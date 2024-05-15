use crate::domain::SharedState;
use crate::errors::ApiError;
use crate::handlers::health_check;
use axum::{
    http::{self, HeaderValue, Method},
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

pub fn build_api_router(app_state: SharedState) -> Result<Router, ApiError> {
    let domain = app_state
        .read()
        .map_err(|_| ApiError::LockPoisoned)?
        .secrets
        .domain
        .clone();
    let parsed_domain = domain
        .parse::<HeaderValue>()
        .map_err(|e| ApiError::BadRequest(format!("Failed to parse domain {}: {}", domain, e)))?;
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![
            http::header::ORIGIN,
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
        ])
        .allow_origin(parsed_domain);

    Ok(Router::new()
        .route("/health_check", get(health_check))
        .layer(cors)
        .with_state(app_state))
}
