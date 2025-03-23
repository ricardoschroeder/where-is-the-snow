use axum::{extract::Query, response::IntoResponse, Json};
use reqwest;
use serde_json;

use crate::models::{WeatherLatLong, WeatherQuery, SnowfallData};

pub async fn snow_forecast_handler(Query(params): Query<WeatherQuery>) -> impl IntoResponse {
    let lat_long = get_city_coordinates_handler(axum::extract::Query(params)).await;
    let url = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,snowfall_sum&timezone=Europe%2FBerlin", lat_long.latitude, lat_long.longitude);

    let response = reqwest::get(&url)
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let snowfall_data: SnowfallData = serde_json::from_value(response["daily"].clone()).unwrap();
        // .iter()
        // .zip(response["daily"]["snowfall_sum"].as_array().unwrap().iter())
        // .map(|(time, snow)| {
        //     (
        //         time.as_str().unwrap_or("").to_string(),
        //         snow.as_f64().unwrap_or(0.0),
        //     )
        // })
        // .collect();

    Json(snowfall_data)
}

pub async fn get_city_coordinates_handler(
    Query(params): Query<WeatherQuery>,
) -> Json<WeatherLatLong> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
        params.city
    );

    let response = reqwest::get(&url)
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    let lat = response["results"][0]["latitude"].as_f64().unwrap_or(0.0);
    let lon = response["results"][0]["longitude"].as_f64().unwrap_or(0.0);

    Json(WeatherLatLong {
        latitude: lat,
        longitude: lon,
    })
}
