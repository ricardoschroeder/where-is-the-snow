use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct WeatherQuery {
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SnowfallData {
    pub time: Vec<String>,
    pub snowfall_sum: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub temperature_2m_max: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherLatLong {
    pub latitude: f64,
    pub longitude: f64,
}