use sqlx::postgres::PgPool;
use weather_app_shared::WeatherData;
use sqlx::types::JsonValue;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(pool: PgPool) -> Result<Self, sqlx::Error> {
        Ok(Self { pool })
    }

    pub async fn log_request(
        &self,
        client_ip: String,
        user_agent: Option<String>,
        search_query: String,
        weather_data: WeatherData,
    ) -> Result<uuid::Uuid, sqlx::Error> {
        let weather_data_json = match serde_json::to_value(&weather_data) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Failed to serialize weather data: {}", e);
                return Err(sqlx::Error::Protocol(e.to_string()));
            }
        };

        println!("Attempting to log request for city: {}", search_query);
        println!("Client IP: {}", client_ip);
        println!("User Agent: {:?}", user_agent);

        match sqlx::query!(
            r#"
            INSERT INTO weather_requests
                (client_ip, user_agent, search_query, weather_data)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            client_ip,
            user_agent,
            search_query,
            weather_data_json as JsonValue
        )
        .fetch_one(&self.pool)
        .await
        {
            Ok(record) => {
                println!("Successfully inserted record with ID: {}", record.id);
                Ok(record.id)
            }
            Err(e) => {
                eprintln!("Database error: {}", e);
                Err(e)
            }
        }
    }

    pub async fn check_recent_entries(&self) -> Result<Vec<(uuid::Uuid, String)>, sqlx::Error> {
        let entries = sqlx::query!(
            r#"
            SELECT id, search_query 
            FROM weather_requests 
            ORDER BY created_at DESC 
            LIMIT 5
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(entries.into_iter().map(|r| (r.id, r.search_query)).collect())
    }
}