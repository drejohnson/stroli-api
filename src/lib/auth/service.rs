use super::{
    models::{AuthenticateRequest, AuthenticateResponse, Signup},
    Pkce,
};
use crate::{domain::types::BuiltinAuthProviderNames, errors::ApiError};
use reqwest::Client;
use std::sync::Arc;

/// AuthService handles the business logic related to user authentication.
#[derive(Clone)]
pub struct AuthService {
    client: Arc<Client>,
    auth_base_url: String,
}

impl AuthService {
    /// Constructs a new instance of `AuthService`.
    pub fn new(client: Arc<Client>, auth_base_url: String) -> Self {
        AuthService {
            client,
            auth_base_url,
        }
    }

    pub async fn process_signup(&self, mut signup: Signup) -> Result<(), ApiError> {
        let pkce = Pkce::generate();
        let signup_url = format!("{}/{}", self.auth_base_url, "register");

        signup.challenge = pkce.challenge;
        signup.verify_url = format!("{}/{}", self.auth_base_url, "verify");

        let response = self.client.post(&signup_url).json(&signup).send().await?;

        dbg!(&response);

        response
            .error_for_status() // Use reqwest helper for error handling
            .map_err(ApiError::from)?;

        Ok(())
    }

    /// Processes user login, expecting an `AuthenticateRequest` and returning an `AuthenticateResponse`.
    pub async fn process_login(
        &self,
        request: &AuthenticateRequest,
    ) -> Result<AuthenticateResponse, ApiError> {
        let login_url = format!("{}/login", self.auth_base_url);
        let response = self
            .client
            .post(&login_url)
            .json(&request)
            .send()
            .await
            .map_err(ApiError::from)?; // Convert network error

        let auth_response = response
            .json::<AuthenticateResponse>()
            .await
            .map_err(ApiError::from)?; // Convert parse error

        Ok(auth_response)
    }

    /// A helper function to handle OAuth login flow for various providers.
    pub async fn process_oauth_login(
        &self,
        provider: BuiltinAuthProviderNames,
        redirect_url: &str,
    ) -> Result<String, ApiError> {
        let oauth_url = format!("{}/oauth/{}", self.auth_base_url, provider.as_str());
        let response = self
            .client
            .get(&oauth_url)
            .query(&[("redirect_url", redirect_url)])
            .send()
            .await
            .map_err(ApiError::from)?;

        response.text().await.map_err(ApiError::from)
    }
}
