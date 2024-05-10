// dependencies
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::ser::Error;
use thiserror::Error;

// an enum to represent possible API error types
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(edgedb_errors::Error),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("JSON parse error: {0}")]
    ParseError(serde_json::Error),
    #[error("Internal server error")]
    InternalServerError,
    #[error("Missing verifier")]
    MissingVerifier,
    #[error("Verification required")]
    VerificationRequired,
}

// implement the From trait for edgedb_errors::Error, for use in the ApiError enum
impl From<edgedb_errors::Error> for ApiError {
    fn from(err: edgedb_errors::Error) -> Self {
        ApiError::Database(err)
    }
}

impl From<url::ParseError> for ApiError {
    fn from(err: url::ParseError) -> Self {
        ApiError::ParseError(serde_json::Error::custom(err.to_string()))
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        // Convert the `reqwest::Error` into an `ApiError` here.
        // This will depend on how your `ApiError` type is defined.
        ApiError::NetworkError(error.to_string())
    }
}

impl From<ApiError> for shuttle_runtime::Error {
    fn from(err: ApiError) -> Self {
        let message = match err {
            ApiError::Database(detail) => format!("Database connection error: {}", detail),
            ApiError::NetworkError(detail) => format!("Network error: {}", detail),
            ApiError::ParseError(detail) => format!("JSON parse error: {}", detail),
            ApiError::InternalServerError => "Internal server error".to_string(),
            ApiError::MissingVerifier => "Missing verifier".to_string(),
            ApiError::VerificationRequired => "Verification required".to_string(),
        };
        shuttle_runtime::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, message))
    }
}

// implement the IntoResponse trait for the ApiError type, for use in returning an error from a handler
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            ApiError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            ApiError::NetworkError(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            ApiError::ParseError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            ApiError::MissingVerifier => (StatusCode::BAD_REQUEST, self.to_string()),
            ApiError::VerificationRequired => (StatusCode::UNAUTHORIZED, self.to_string()),
        };

        // Optionally, log the error here
        eprintln!("Error encountered: {}", msg); // Replace with a proper logging mechanism in production

        (status, msg).into_response()
    }
}
