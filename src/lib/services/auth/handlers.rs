use crate::domain::state::SharedState;
use crate::services::auth::error::Result;
use crate::services::auth::{
    commands,
    model::{AuthResponse, SigninParams, SignupParams},
};
use axum::http::StatusCode;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar, SameSite};
use time::Duration;

pub async fn register_user(
    State(state): State<SharedState>,
    Json(payload): Json<SignupParams>,
) -> Result<impl IntoResponse> {
    let token = commands::register_user(&state, payload).await?;
    Ok(Json(AuthResponse {
        message: "User registered successfully".to_string(),
        token: Some(token),
    }))
}

pub async fn login_user(
    State(state): State<SharedState>,
    Json(payload): Json<SigninParams>,
) -> Result<impl IntoResponse> {
    let token = commands::login_user(&state, payload).await?;

    // jar.add(cookie);

    Ok(Json(AuthResponse {
        message: "Login successful".to_string(),
        token: Some(token),
    }))
}
