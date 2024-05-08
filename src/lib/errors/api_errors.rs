// dependencies
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

// an enum to represent possible API error types
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Database error: {0}")]
    Database(surrealdb::Error),
    #[error("Network error: {0}")]
    NetworkError(String),
}

// implement the From trait for surrealdb::Error, for use in the ApiError enum
impl From<surrealdb::Error> for ApiError {
    fn from(err: surrealdb::Error) -> Self {
        ApiError::Database(err)
    }
}

impl From<ApiError> for shuttle_runtime::Error {
    fn from(err: ApiError) -> Self {
        let message = match err {
            ApiError::Database(detail) => format!("Database connection error: {}", detail),
            ApiError::NetworkError(detail) => format!("Network error: {}", detail),
        };
        shuttle_runtime::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, message))
    }
}

// implement the IntoResponse trait for the ApiError type, for use in returning an error from a handler
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error");
        (status, msg).into_response()
    }
}
