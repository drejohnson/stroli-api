use crate::domain::state::SharedState;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
};
use http::request::Parts;
use serde::{Deserialize, Serialize};

use crate::services::auth::middleware::error::{Error, Result};
use crate::services::auth::middleware::util::extract_and_verify_token;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Authorization {
    pub user_id: String,
    pub token: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Authorization
where
    S: Send + Sync,
    SharedState: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let state = SharedState::from_ref(state);

        let (claims, token) = extract_and_verify_token(&parts.headers, state).await?;

        Ok(Authorization {
            user_id: claims.id, // claims.sub,
            token,
        })
    }
}
