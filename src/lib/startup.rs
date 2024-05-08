// dependencies
use crate::domain::SharedState;

use crate::handlers::health_check;
use axum::{
    http::{self},
    middleware::{self},
    routing::get,
    Router,
};
use http::header::{ACCEPT, AUTHORIZATION, ORIGIN};
use http::HeaderValue;
use http::Method;
use tower_http::cors::CorsLayer;

// function which configures and returns an AxumService
pub fn build_api_router(app_state: SharedState) -> Router {
    let domain = app_state.read().unwrap().secrets.domain.clone();
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
        .allow_origin(domain.parse::<HeaderValue>().unwrap());
    // create a new router
    Router::new()
        // .layer(middleware::from_fn_with_state(
        //     app_state.clone(),
        //     app_state.read().unwrap().key,
        // ))
        // add a health_check endpoint
        .route("/health_check", get(health_check))
        // add a CORS layer to the router
        .layer(cors)
        .with_state(app_state)
}
