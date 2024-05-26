use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use tokio::task;

use crate::{
    domain::secrets::AppSecrets,
    services::auth::{keys, model::Claims},
};

use crate::services::auth::utils::error::{Error, Result};

// pub fn sign(
//     state: &SharedState,
//     id: sql::Thing,
//     sc: &str,
//     tk: &str,
//     email: &str,
// ) -> Result<(EncodingKey, Claims)> {
//     let now = OffsetDateTime::now_utc();
//     let exp = Duration::days(1);
//     let claims = Claims {
//         sub: id.to_string(),
//         iat: now.unix_timestamp() as usize,
//         iss: state.secrets.jwt_issuer.clone(),
//         exp: (now + exp).unix_timestamp() as usize,
//         aud: format!("{}:{}", state.secrets.jwt_issuer, state.secrets.jwt_name),
//         ns: state.secrets.db_namespace.clone(),
//         db: state.secrets.db_database_name.clone(),
//         sc: sc.to_string(),
//         id: id.to_string(),
//         tk: tk.to_string(),
//         email: email.to_string(),
//     };

//     let keys = keys::get_keys(&state.secrets);

//     let token = encode(
//         &Header::new(Algorithm::RS256),
//         &claims,
//         &EncodingKey::from_rsa_pem(keys.private.as_bytes()).map_err(|e| e.into()),
//     );

//     (token.unwrap(), claims)
// }

pub async fn sign_token(secrets: &AppSecrets, claims: Claims) -> Result<String> {
    let header = Header::new(Algorithm::RS256);
    let keys = keys::get_keys(&secrets);

    let encoding_key = match EncodingKey::from_rsa_pem(keys.private.as_bytes()) {
        Ok(key) => key,
        Err(err) => return Err(Error::from(err)),
    };

    task::spawn_blocking(move || encode(&header, &claims, &encoding_key).map_err(Error::from))
        .await
        .map_err(|e| Error::JoinError(e.to_string()))?
}
