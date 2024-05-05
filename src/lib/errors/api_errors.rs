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
    Database(edgedb_errors::Error),
    // TODO: add more error types here as needed
}

// implement the From trait for edgedb_errors::Error, for use in the ApiError enum
impl From<edgedb_errors::Error> for ApiError {
    fn from(err: edgedb_errors::Error) -> Self {
        ApiError::Database(err)
    }
}

// implement the IntoResponse trait for the ApiError type, for use in returning an error from a handler
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error");
        (status, msg).into_response()
    }
}
