use axum::{Extension, Json};
use axum_macros::debug_handler;

use crate::auth::models::{AuthenticateRequest, Signup};
use crate::auth::service::AuthService;
use crate::errors::ApiError;

use super::AuthenticateResponse;

pub async fn signup_handler(
    Extension(auth_service): Extension<AuthService>,
    Json(payload): Json<Signup>,
) -> Result<Json<()>, ApiError> {
    dbg!(&payload);
    auth_service.process_signup(payload).await.map(Json)
}

#[debug_handler]
pub async fn login_handler(
    Extension(auth_service): Extension<AuthService>,
    Json(payload): Json<AuthenticateRequest>,
) -> Result<Json<AuthenticateResponse>, ApiError> {
    dbg!(&payload);
    auth_service.process_login(&payload).await.map(Json)
}
