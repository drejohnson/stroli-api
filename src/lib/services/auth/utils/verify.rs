use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use tokio::task;

use crate::{
    domain::state::SharedState,
    services::auth::{keys, model::Claims},
};

use crate::services::auth::utils::error::{Error, Result};

pub async fn verify_token(state: SharedState, token: &str) -> Result<Claims> {
    let keys = keys::get_keys(&state.secrets);

    let decoding_key = match DecodingKey::from_rsa_pem(keys.public.as_bytes()) {
        Ok(key) => key,
        Err(err) => return Err(Error::from(err)),
    };

    let token = token.to_string(); // Clone the token to move it into the async block

    match task::spawn_blocking(move || {
        jsonwebtoken::decode::<Claims>(&token, &decoding_key, &Validation::new(Algorithm::RS256))
            .map(|data| data.claims)
            .map_err(Error::from)
    })
    .await
    {
        Ok(Ok(claims)) => Ok(claims),
        Ok(Err(err)) => Err(err),
        Err(join_err) => Err(Error::JoinError(join_err.to_string())), // Handle JoinError
    }
}
