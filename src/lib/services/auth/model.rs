use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Jwt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,  // Issued at
    pub iss: String, // Issuer
    pub exp: usize,  // Expiry time
    pub aud: String, // Audience
    #[serde(rename = "NS")]
    pub ns: String, // Namespace
    #[serde(rename = "DB")]
    pub db: String, // Database
    #[serde(rename = "SC")]
    pub sc: String, // Scope
    #[serde(rename = "ID")]
    pub id: String, // User ID
    #[serde(rename = "TK")]
    pub tk: String, // Token
    pub email: String, // Email
}

#[derive(Serialize, Deserialize)]
pub struct SignupParams {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SigninParams {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
    pub token: Option<Jwt>,
}
