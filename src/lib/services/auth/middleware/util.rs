use crate::domain::state::SharedState;
use crate::services::auth::middleware::error::{Error, Result};
use crate::services::auth::model::Claims;
use crate::services::auth::utils::verify::verify_token;
use http::HeaderMap;

pub async fn extract_and_verify_token(
    headers: &HeaderMap,
    state: SharedState,
) -> Result<(Claims, String)> {
    let auth_header = headers
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or(Error::MissingAuthorizationHeader)?;

    let (prefix, token) = auth_header
        .split_once(' ')
        .ok_or(Error::InvalidAuthorizationHeaderFormat)?;

    if prefix != "Bearer" {
        return Err(Error::InvalidAuthorizationHeaderFormat);
    }

    let claims = match verify_token(state, token).await {
        Ok(claims) => claims,
        Err(e) => return Err(Error::InvalidToken(e.to_string())),
    };

    Ok((claims, token.to_string()))
}
