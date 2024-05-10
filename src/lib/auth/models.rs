use super::super::domain::types::BuiltinAuthProviderNames;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDetails {
    pub provider: BuiltinAuthProviderNames,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signup {
    #[serde(flatten)]
    pub credentials: UserCredentials,
    #[serde(flatten)]
    pub auth_details: AuthDetails,
    pub name: String,
    pub username: String,
    pub challenge: String,
    pub verify_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticateRequest {
    #[serde(flatten)]
    pub auth_details: AuthDetails,
    #[serde(flatten)]
    pub credentials: UserCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticateResponse {
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleAuthorize {
    pub provider: BuiltinAuthProviderNames,
}
