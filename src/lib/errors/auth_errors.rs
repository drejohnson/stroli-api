use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl From<AuthError> for shuttle_runtime::Error {
    fn from(err: AuthError) -> Self {
        let message = match err {
            AuthError::WrongCredentials => "Wrong credentials".to_string(),
            AuthError::MissingCredentials => "Missing credentials".to_string(),
            AuthError::TokenCreation => "Token creation error".to_string(),
            AuthError::InvalidToken => "Invalid token".to_string(),
        };
        shuttle_runtime::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, message))
    }
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
