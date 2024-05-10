use crate::auth::{login_handler, signup_handler, AuthService};
// dependencies
use crate::domain::SharedState;

use crate::handlers::health_check;
use axum::Extension;
use axum::{
    http,
    routing::{get, post},
    Router,
};
use http::header::{ACCEPT, AUTHORIZATION, ORIGIN};
use http::HeaderValue;
use http::Method;
use tower_http::cors::CorsLayer;

pub fn build_api_router(app_state: SharedState, auth_service: AuthService) -> Router {
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![ORIGIN, AUTHORIZATION, ACCEPT])
        .allow_origin(
            app_state
                .read()
                .unwrap()
                .secrets
                .domain
                .parse::<HeaderValue>()
                .unwrap(),
        );

    let auth_router = Router::new()
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
        .layer(Extension(auth_service));

    Router::new()
        .nest("/api/auth", auth_router)
        .route("/api/health_check", get(health_check))
        .with_state(app_state)
        .layer(cors)
}
