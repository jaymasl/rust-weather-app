use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WeatherData {
    pub location: Location,
    pub current: Current,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Current {
    pub last_updated_epoch: i64,
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i32,
    pub condition: Condition,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: i32,
    pub wind_dir: String,
    pub pressure_mb: f64,
    pub pressure_in: f64,
    pub precip_mm: f64,
    pub precip_in: f64,
    pub humidity: i32,
    pub cloud: i32,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub uv: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
    pub air_quality: Option<AirQuality>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Condition {
    pub text: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AirQuality {
    pub co: f64,
    pub no2: f64,
    pub o3: f64,
    pub so2: f64,
    #[serde(rename = "pm2_5")]
    pub pm2_5: f64,
    pub pm10: f64,
    #[serde(rename = "us-epa-index")]
    pub us_epa_index: i32,
}