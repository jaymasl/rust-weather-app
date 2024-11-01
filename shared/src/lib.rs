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
    #[serde(default)]
    pub air_quality: Option<AirQuality>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Condition {
    pub text: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AirQuality {
    #[serde(default)]
    pub co: Option<f64>,
    #[serde(default)]
    pub no2: Option<f64>,
    #[serde(default)]
    pub o3: Option<f64>,
    #[serde(default)]
    pub so2: Option<f64>,
    #[serde(rename = "pm2_5", default)]
    pub pm2_5: Option<f64>,
    #[serde(default)]
    pub pm10: Option<f64>,
    #[serde(rename = "us-epa-index", default)]
    pub us_epa_index: Option<i32>,
}

impl AirQuality {
    pub fn is_complete(&self) -> bool {
        self.co.is_some() 
            && self.no2.is_some() 
            && self.o3.is_some() 
            && self.so2.is_some() 
            && self.pm2_5.is_some() 
            && self.pm10.is_some() 
            && self.us_epa_index.is_some()
    }

    pub fn get_epa_index(&self) -> Option<i32> {
        self.us_epa_index
    }
}