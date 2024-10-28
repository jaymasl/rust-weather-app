use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use weather_app_shared::WeatherData;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherRequest {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub client_ip: String,
    pub user_agent: Option<String>,
    pub search_query: String,
    pub weather_data: WeatherData,
}

impl WeatherRequest {
    pub fn new(
        client_ip: String,
        user_agent: Option<String>,
        search_query: String,
        weather_data: WeatherData,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            client_ip,
            user_agent,
            search_query,
            weather_data,
        }
    }
}