use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid Authorization Header Format")]
    InvalidAuthorizationHeaderFormat,
    #[error("Authorization Header Missing")]
    MissingAuthorizationHeader,
    #[error("Invalid Token: {0}")]
    InvalidToken(String),
    #[error("InvalidClaims: {0}")]
    InvalidClaims(String),
    #[error("Surreal Error: {0}")]
    SurrealError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        error!("{:<12} - query::Error {:?}", "INTO_RES", &self);

        let (status, error_message) = match &self {
            Error::InvalidAuthorizationHeaderFormat => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::MissingAuthorizationHeader => (StatusCode::BAD_REQUEST, self.to_string()),
            Error::InvalidToken(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::InvalidClaims(_) => (StatusCode::UNAUTHORIZED, self.to_string()),
            Error::SurrealError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
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

impl From<Error> for shuttle_runtime::Error {
    fn from(err: Error) -> Self {
        shuttle_runtime::Error::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            err.to_string(),
        ))
    }
}

impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Self {
        Error::SurrealError(err.to_string())
    }
}
