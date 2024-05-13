use crate::auth::keys::KEYS;
use crate::auth::models::Claims;
use crate::domain::AppState;
use crate::errors::AuthError;
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{self, encode, Algorithm, Header};
use std::sync::{Arc, RwLock};
use time::{Duration, OffsetDateTime};

pub fn generate_user_token(
    app_state: Arc<RwLock<AppState>>,
    sc: &str,
    tk: &str,
    id: &str,
    secure: Option<bool>,
) -> Result<(String, String), AuthError> {
    let state = app_state.read().unwrap();
    let now = OffsetDateTime::now_utc();
    let exp = Duration::hours(24);
    let claims = Claims {
        iat: now.unix_timestamp() as usize,
        iss: state.secrets.jwt_issuer.clone(),
        exp: (now + exp).unix_timestamp() as usize,
        aud: format!("{}:{}", state.secrets.jwt_issuer, state.secrets.jwt_name),
        ns: state.secrets.db_namespace.clone(),
        db: state.secrets.db_database_name.clone(),
        sc: sc.to_string(),
        id: id.to_string(),
        tk: tk.to_string(),
        email: "".to_string(),
    };

    // Encode the token using the secret key.
    let token = encode(&Header::new(Algorithm::HS256), &claims, &KEYS.encoding).map_err(|e| {
        eprintln!("Failed to create token: {:?}", e);
        AuthError::TokenCreation
    })?;

    let cookie = Cookie::build((state.secrets.jwt_name.clone(), token.clone()))
        .path("/api")
        .http_only(true)
        .same_site(SameSite::Strict)
        .secure(secure.unwrap_or(true))
        .max_age(exp);

    Ok((token, cookie.to_string()))
}
