use axum::{http::StatusCode, response::IntoResponse};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailFlow {
    email: String,
    verify_url: String,
}

// health_check endpoint handler
#[debug_handler]
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
