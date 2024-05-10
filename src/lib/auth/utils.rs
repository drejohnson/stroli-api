// src/pkce.rs

use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::{
    cookie::{Cookie, PrivateCookieJar},
    CookieJar,
};
use reqwest;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    domain::{AppState, BuiltinAuthProviderNames},
    errors::ApiError,
};

use super::Pkce;

async fn send_http_request<T: for<'de> Deserialize<'de>, U: Serialize>(
    client: &reqwest::Client,
    method: reqwest::Method,
    url: &str,
    body: Option<&U>,
) -> Result<T, ApiError> {
    client
        .request(method, url)
        .json(&Some(body).or(None))
        .send()
        .await
        .map_err(|e| ApiError::NetworkError(e.to_string()))?
        .json::<T>()
        .await
        .map_err(|e| ApiError::NetworkError(e.to_string()))
}

#[derive(Debug, Deserialize)]
pub struct HandleAuthorize {
    provider: String,
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

// Handle the oAuth2 authorize request
pub async fn handle_authorize(
    State(state): State<AppState>,
    Query(query): Query<HandleAuthorize>,
    jar: PrivateCookieJar,
) -> Result<impl IntoResponse, ApiError> {
    let HandleAuthorize { provider } = query;
    let pkce = Pkce::generate();
    let cookie = Cookie::new("edgedb-pkce-verifier", pkce.verifier);

    let redirect_url = match build_url(
        &state,
        "authorize",
        &[
            ("provider", &provider),
            ("challenge", &pkce.challenge),
            (
                "redirect_to",
                &format!("{}/{}", state.secrets.edgedb_base_auth_url, "callback"),
            ),
        ],
    ) {
        Ok(url) => url,
        Err(_) => return Err(ApiError::NetworkError("Failed to build URL".to_string())),
    };

    let _ = jar.add(cookie);

    Ok(Redirect::permanent(&redirect_url))
}

//
// EMAIL AND PASSWORD FLOW: https://www.edgedb.com/docs/guides/auth/email_password
// See https://www.edgedb.com/docs/guides/auth/index#enabling-authentication-providers for config

//  POST /signup
#[derive(Debug, Deserialize)]
pub struct EmailFlowQuery {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct Signup {
    challenge: String,
    email: String,
    password: String,
    provider: BuiltinAuthProviderNames,
    verify_url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegistrationResponse {
    code: Option<String>,
}

pub async fn handle_email_password_signup(
    State(state): State<AppState>,
    Query(query): Query<EmailFlowQuery>,
    jar: PrivateCookieJar,
) -> Result<(impl IntoResponse, StatusCode), ApiError> {
    let pkce = Pkce::generate();

    let EmailFlowQuery { email, password } = query;

    let register_url = build_url(&state, "register", &[])?;

    let signup = Signup {
        challenge: pkce.challenge,
        email,
        password,
        provider: BuiltinAuthProviderNames::EmailPassword.clone(),
        verify_url: format!("{}/{}", state.secrets.edgedb_base_auth_url, "verify"),
    };

    let client = reqwest::Client::new();
    let response: RegistrationResponse =
        send_http_request(&client, reqwest::Method::POST, &register_url, Some(&signup)).await?;

    // Pattern match on the `code` field of the response.
    match response.code {
        Some(code) => {
            println!("Received code: {}", code);
            let token_url = build_url(
                &state,
                "token",
                &[("code", &code), ("verifier", &pkce.verifier)],
            )?;
            let token_data = reqwest::get(Url::parse(&token_url)?).await?.text().await?;
            let cookie = Cookie::new("edgedb-pkce-verifier", pkce.verifier);
            let _ = jar.add(cookie);
            Ok((token_data, StatusCode::OK))
        }
        None => Err(ApiError::VerificationRequired),
    }
}

// POST /signin
#[derive(Debug, Serialize, Deserialize)]
struct AuthenticateRequest {
    challenge: String,
    email: String,
    password: String,
    provider: BuiltinAuthProviderNames,
}

#[derive(Debug, Serialize, Deserialize)]
struct AuthenticateResponse {
    code: String,
}

pub async fn handle_email_password_signin(
    State(state): State<AppState>,
    Query(query): Query<EmailFlowQuery>,
    jar: PrivateCookieJar,
) -> Result<impl IntoResponse, ApiError> {
    let pkce = Pkce::generate();
    println!("Got a signin: {query:?}");
    let EmailFlowQuery { email, password } = query;

    let authenticate_url = format!("{}/{}", state.secrets.edgedb_base_auth_url, "authenticate");

    let authenticate_request = AuthenticateRequest {
        challenge: pkce.challenge,
        email,
        password,
        provider: BuiltinAuthProviderNames::EmailPassword.clone(),
    };

    let client = reqwest::Client::new();
    let response: AuthenticateResponse = send_http_request(
        &client,
        reqwest::Method::POST,
        &authenticate_url,
        Some(&authenticate_request),
    )
    .await?;

    let token_url = build_url(
        &state,
        "token",
        &[("code", &response.code), ("verifier", &pkce.verifier)],
    )?;

    let auth_token = reqwest::get(Url::parse(&token_url)?).await?.text().await?;

    println!("Auth token: {auth_token}");

    let cookie = Cookie::new("edgedb-auth-token", auth_token);
    let _ = jar.add(cookie);

    Ok(StatusCode::NO_CONTENT)
}

//  GET /verify
#[derive(Debug, Deserialize)]
pub struct HandleVerify {
    verification_token: String,
}

#[derive(Debug, Serialize)]
pub struct Verify {
    verification_token: String,
    verifier: String,
    provider: BuiltinAuthProviderNames,
}

pub async fn handle_verify(
    State(state): State<AppState>,
    Query(query): Query<HandleVerify>,
    Extension(jar): Extension<CookieJar>,
) -> Result<impl IntoResponse, ApiError> {
    let HandleVerify { verification_token } = query;

    // Get verifier from cookie jar
    let verifier = jar
        .get("edgedb-pkce-verifier")
        .map(|cookie| cookie.value().to_string())
        .ok_or_else(|| ApiError::MissingVerifier)?; // Custom error for missing verifier

    let verify_url = build_url(
        &state,
        "verify",
        &[
            ("verification_token", &verification_token),
            ("verifier", &verifier),
            ("provider", BuiltinAuthProviderNames::EmailPassword.as_str()),
        ],
    )?;

    let verify = Verify {
        verification_token,
        verifier: verifier.clone(),
        provider: BuiltinAuthProviderNames::EmailPassword.clone(),
    };

    let client = reqwest::Client::new();
    let response: String =
        send_http_request(&client, reqwest::Method::POST, &verify_url, Some(&verify)).await?;

    dbg!("Verify response: {}", response);

    let cookie = Cookie::new("edgedb-pkce-verifier", verifier);
    let updated_cookie = jar.add(cookie);

    Ok((updated_cookie, StatusCode::NO_CONTENT))
}
