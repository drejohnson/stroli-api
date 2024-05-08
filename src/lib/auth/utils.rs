// src/pkce.rs

use axum::{
    extract::{Extension, Query, Request, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, PrivateCookieJar};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use rand::Rng;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use url::Url;

use crate::domain::AppState;

#[derive(Debug, Deserialize)]
pub struct HandleAuthorize {
    provider: String,
}

pub struct Pkce {
    pub verifier: String,
    pub challenge: String,
}

impl Pkce {
    pub fn generate() -> Self {
        let random_bytes: [u8; 32] = rand::thread_rng().gen();
        const CUSTOM_ENGINE: engine::GeneralPurpose =
            engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
        let verifier = CUSTOM_ENGINE.encode(&random_bytes);

        let mut hasher = Sha256::new();
        hasher.update(&random_bytes);
        let challenge_bytes = hasher.finalize();
        let challenge = CUSTOM_ENGINE.encode(&challenge_bytes);

        Self {
            verifier,
            challenge,
        }
    }
}

pub fn build_url(
    state: &AppState,
    suffix: &str,
    params: &[(&str, &str)],
) -> Result<String, url::ParseError> {
    let base_url = format!("{}/{}", state.secrets.edgedb_base_auth_url, suffix);
    let mut url = Url::parse(&base_url)?;

    for &(name, value) in params {
        url.query_pairs_mut().append_pair(name, value);
    }

    Ok(url.to_string())
}

pub async fn handle_authorize(
    State(state): State<AppState>,
    Query(query): Query<HandleAuthorize>,
    jar: PrivateCookieJar,
) -> Result<Redirect, (StatusCode, String)> {
    let HandleAuthorize { provider } = query.into();
    let pkce = Pkce::generate();

    let redirect_url = match build_url(
        &state,
        "authorize",
        &[
            ("provider", &provider),
            ("challenge", &pkce.challenge),
            (
                "redirect_to",
                &format!("{}/{}", state.secrets.edgedb_base_auth_url, "/callback"),
            ),
        ],
    ) {
        Ok(url) => url,
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to build URL".into(),
            ))
        }
    };

    let cookie = Cookie::new("edgedb-pkce-verifier", pkce.verifier);
    let _ = jar.add(cookie);

    Ok(Redirect::permanent(&redirect_url))
}
