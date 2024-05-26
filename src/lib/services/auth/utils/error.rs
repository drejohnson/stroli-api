use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Auth service error: {0}")]
    ServiceSpecificError(String),
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("JWT error: {0}")]
    JWTError(String),
    #[error("Join error: {0}")]
    JoinError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{:<12} - query::Error {:?}", "INTO_RES", &self);

        let (status, error_message) = match &self {
            Error::ServiceSpecificError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::InvalidCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::JWTError(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::JoinError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let error_response = ErrorResponse {
            code: status.as_u16(),
            error: error_message,
        };

        let response = (status, Json(error_response)).into_response();

        error!("{:?}", &response);

        response
    }
}

impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Self {
        Error::ServiceSpecificError(err.to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Error::JWTError(err.to_string())
    }
}

impl From<Error> for shuttle_runtime::Error {
    fn from(err: Error) -> Self {
        shuttle_runtime::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            err.to_string(),
        ))
    }
}
