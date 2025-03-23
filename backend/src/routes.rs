use axum::{
    routing::get,
    Router,
};

use crate::handler::{snow_forecast_handler, get_city_coordinates_handler};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/snowforecast", get(snow_forecast_handler))
        .route("/api/coordinates", get(get_city_coordinates_handler))
}