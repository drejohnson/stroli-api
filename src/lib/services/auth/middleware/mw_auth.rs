use crate::domain::state::SharedState;
use crate::services::auth::middleware::error::Result;
use crate::services::auth::middleware::util::extract_and_verify_token;
use axum::{extract::State, http::Request, middleware::Next, response::Response};

pub async fn auth_middleware(
    State(state): State<SharedState>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response> {
    let (claims, _token) = extract_and_verify_token(req.headers(), state).await?;

    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
