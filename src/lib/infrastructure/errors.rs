use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::ser::Error;
use serde::Serialize;

use tracing::{error, instrument};

use crate::infrastructure::db;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    DatabaseError(#[from] db::Error),
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Lock poisoned")]
    LockPoisoned,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("JSON parse error: {0}")]
    ParseError(serde_json::Error),
    #[error("Internal server error")]
    InternalServerError(String),
    #[error("Not found: {0}")]
    NotFound(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    error: String,
}

impl From<url::ParseError> for ApiError {
    fn from(err: url::ParseError) -> Self {
        ApiError::ParseError(serde_json::Error::custom(err.to_string()))
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::NetworkError(error.to_string())
    }
}

impl IntoResponse for ApiError {
    #[instrument]
    fn into_response(self) -> Response {
        let (status, response) = self.status_and_message();

        error!("Error encountered: {}", response.message); // Use tracing for structured logging

        (status, Json(response)).into_response()
    }
}

impl ApiError {
    fn status_and_message(&self) -> (StatusCode, ErrorResponse) {
        match self {
            Self::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.error_response("Database error"),
            ),
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, self.error_response("Bad request")),
            Self::LockPoisoned => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.error_response("Lock poisoned"),
            ),
            Self::NetworkError(_) => (
                StatusCode::BAD_GATEWAY,
                self.error_response("Network error"),
            ),
            Self::ParseError(_) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                self.error_response("JSON parse error"),
            ),
            Self::InternalServerError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                self.error_response("Internal server error"),
            ),
            Self::NotFound(_) => (StatusCode::NOT_FOUND, self.error_response("Not found")),
        }
    }

    fn error_response(&self, message: &str) -> ErrorResponse {
        ErrorResponse {
            code: self.status_and_message().0.as_u16(),
            message: message.to_string(),
            error: self.to_string(),
        }
    }
}
