use axum::{routing::post, Router};

use crate::{domain::state::SharedState, services::auth::handlers};

pub fn auth_routes(state: SharedState) -> Router {
    Router::new()
        .route("/auth/register", post(handlers::register_user))
        .route("/auth/login", post(handlers::login_user))
        .with_state(state)
}
