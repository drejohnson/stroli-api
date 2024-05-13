use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
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

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub sc: String,
    pub id: String,
    pub email: String,
    pub secure: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenValidation {
    pub valid: bool,
    pub claims: Option<Claims>,
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

// allow us to print the claim details for the private route
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}\nIssuer: {}", self.id, self.iss)
    }
}

// implement a method to create a response type containing the JWT
impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

impl TokenValidation {
    pub fn new(valid: bool, claims: Option<Claims>) -> Self {
        TokenValidation { valid, claims }
    }
}
