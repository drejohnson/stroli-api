use axum_extra::extract::cookie::{Cookie, SameSite};
use surrealdb::opt::auth::Jwt;
use time::Duration;

use crate::{domain::state::SharedState, services::auth::error::Result};

pub fn generate_cookie(state: &SharedState, token: &Jwt, secure: Option<bool>) -> Result<String> {
    let cookie = Cookie::build((&state.secrets.jwt_name, token.as_insecure_token()))
        .path("/api")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(secure.unwrap_or(true))
        .max_age(Duration::days(1));

    Ok(cookie.to_string())
}
