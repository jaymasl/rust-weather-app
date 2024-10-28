#[macro_use] extern crate rocket;

mod db;
use db::Database;
use rocket::{State, http::Status, serde::json::Json};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::fs::FileServer;
use reqwest::Client;
use weather_app_shared::WeatherData;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::{Instant, Duration};
use tokio::sync::Mutex;
use shuttle_runtime::SecretStore;
use include_dir::{include_dir, Dir};

static STATIC_DIR: Dir<'_> = include_dir!("static");

struct RateLimiter {
    requests: HashMap<String, Instant>,
}

impl RateLimiter {
    fn new() -> Self {
        RateLimiter {
            requests: HashMap::new(),
        }
    }

    fn check_rate_limit(&mut self, ip: &str) -> bool {
        let now = Instant::now();
        if let Some(last_request) = self.requests.get(ip) {
            if now.duration_since(*last_request) < Duration::from_secs(5) {
                return false;
            }
        }
        self.requests.insert(ip.to_string(), now);
        true
    }
}

#[derive(Debug, Clone)]
struct RequestMetadata {
    ip: String,
    user_agent: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestMetadata {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let ip = request.headers()
            .get_one("X-Forwarded-For")
            .map(|forwarded| {
                forwarded.split(',')
                    .next()
                    .unwrap_or("unknown")
                    .trim()
                    .to_string()
            })
            .or_else(|| {
                request.headers()
                    .get_one("X-Real-IP")
                    .map(String::from)
            })
            .or_else(|| {
                request.client_ip()
                    .map(|ip| ip.to_string())
            })
            .unwrap_or_else(|| "unknown".to_string());
            
        let user_agent = request.headers()
            .get_one("User-Agent")
            .map(String::from);

        Outcome::Success(RequestMetadata { ip, user_agent })
    }
}

struct AppState {
    weather_api_key: Arc<str>,
    http_client: Client,
    database: Database,
    rate_limiter: Arc<Mutex<RateLimiter>>,
}

#[shuttle_runtime::main]
async fn rocket(
   #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
   #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_rocket::ShuttleRocket {
    let weather_api_key: Arc<str> = secrets
        .get("WEATHER_API_KEY")
        .expect("WEATHER_API_KEY must be set in Secrets.toml")
        .into();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let database = Database::new(pool)
        .await
        .expect("Failed to initialize database");

    let http_client = Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(15))
        .pool_max_idle_per_host(10)
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("Failed to build HTTP client");

    let cors = rocket_cors::CorsOptions {
        allowed_origins: rocket_cors::AllowedOrigins::all(),
        allowed_methods: vec![
            rocket::http::Method::Get,
            rocket::http::Method::Post,
            rocket::http::Method::Options
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        expose_headers: ["Content-Type", "Accept"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        allow_credentials: true,
        max_age: Some(3600),
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS configuration");

    let temp_dir = std::env::temp_dir().join(format!("weather_app_static_{}", uuid::Uuid::new_v4()));
    
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir).expect("Failed to clean existing temp directory");
    }
    std::fs::create_dir_all(&temp_dir).expect("Failed to create temp directory");
    
    STATIC_DIR.extract(&temp_dir).expect("Failed to extract static files");

    println!("Static files path: {:?}", temp_dir);

    let rocket_builder = rocket::build()
        .mount("/", FileServer::from(temp_dir))
        .mount("/api", routes![get_weather, recent_entries, options_handler])
        .manage(AppState {
            weather_api_key,
            http_client,
            database,
            rate_limiter: Arc::new(Mutex::new(RateLimiter::new())),
        })
        .register("/", catchers![not_found])
        .attach(cors);

    Ok(rocket_builder.into())
}

#[get("/weather/<city>")]
async fn get_weather(
    city: &str, 
    state: &State<AppState>,
    metadata: RequestMetadata,
) -> Result<Json<WeatherData>, Status> {
    println!("Received weather request for city: {}", city);
    
    let mut rate_limiter = state.rate_limiter.lock().await;
    if !rate_limiter.check_rate_limit(&metadata.ip) {
        println!("Rate limit exceeded for IP: {}", metadata.ip);
        return Err(Status::TooManyRequests);
    }
    drop(rate_limiter);

    let mut url = String::with_capacity(128);
    url.push_str("https://api.weatherapi.com/v1/current.json?key=");
    url.push_str(&state.weather_api_key);
    url.push_str("&q=");
    url.push_str(city);
    url.push_str("&aqi=yes");

    println!("Sending request to weather API");

    match state.http_client.get(&url).send().await {
        Ok(response) => {
            println!("Weather API response status: {}", response.status());
            if response.status().is_success() {
                match response.json::<WeatherData>().await {
                    Ok(weather_data) => {
                        println!("Successfully parsed weather data");
                        
                        if let Err(e) = state.database.log_request(
                            metadata.ip,
                            metadata.user_agent,
                            city.to_string(),
                            weather_data.clone(),
                        ).await {
                            eprintln!("Failed to log request: {}", e);
                        }
                        
                        Ok(Json(weather_data))
                    },
                    Err(e) => {
                        eprintln!("Failed to parse weather data: {}", e);
                        Err(Status::InternalServerError)
                    }
                }
            } else {
                eprintln!("Weather API error: {}", response.status());
                Err(Status::NotFound)
            }
        },
        Err(e) => {
            eprintln!("Request error: {}", e);
            Err(Status::NotFound)
        }
    }
}

#[get("/debug/recent")]
async fn recent_entries(state: &State<AppState>) -> Result<Json<Vec<(uuid::Uuid, String)>>, Status> {
    match state.database.check_recent_entries().await {
        Ok(entries) => Ok(Json(entries)),
        Err(e) => {
            eprintln!("Failed to fetch recent entries: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[options("/<_..>")]
async fn options_handler() -> Status {
    println!("Handling OPTIONS request");
    Status::Ok
}

#[catch(404)]
fn not_found() -> Status {
    Status::NotFound
}