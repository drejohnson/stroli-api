use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::ser::Error;
use surrealdb::Error as SurrealError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Lock poisoned")]
    LockPoisoned,
    #[error("Database error: {0}")]
    Database(SurrealError),
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
    #[error("NotFound: {0}")]
    NotFound(String),
    #[error("RDKafka error: {0}")]
    RDKafka(#[from] rdkafka::error::RDKafkaError),
    #[error("Kafka error: {0}")]
    Kafka(rdkafka::error::KafkaError),
    #[error("De/serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Oneshot message was canceled")]
    CanceledMessage(#[from] futures::channel::oneshot::Canceled),
}

impl From<SurrealError> for ApiError {
    fn from(err: SurrealError) -> Self {
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
        ApiError::NetworkError(error.to_string())
    }
}

impl From<ApiError> for shuttle_runtime::Error {
    fn from(err: ApiError) -> Self {
        let message = match err {
            ApiError::BadRequest(detail) => format!("Bad request: {}", detail),
            ApiError::LockPoisoned => "Lock poisoned".to_string(),
            ApiError::Database(detail) => format!("Database connection error: {}", detail),
            ApiError::NetworkError(detail) => format!("Network error: {}", detail),
            ApiError::ParseError(detail) => format!("JSON parse error: {}", detail),
            ApiError::InternalServerError => "Internal server error".to_string(),
            ApiError::MissingVerifier => "Missing verifier".to_string(),
            ApiError::VerificationRequired => "Verification required".to_string(),
            ApiError::NotFound(detail) => format!("Not found: {}", detail),
            ApiError::RDKafka(detail) => format!("RDKafka error: {}", detail),
            ApiError::Kafka(detail) => format!("Kafka error: {}", detail),
            ApiError::SerdeJson(detail) => format!("De/serialization error: {}", detail),
            ApiError::CanceledMessage(_) => "Oneshot message was canceled".to_string(),
        };
        shuttle_runtime::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, message))
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            Self::BadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::LockPoisoned => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            Self::NetworkError(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            Self::ParseError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Self::MissingVerifier => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::VerificationRequired => (StatusCode::UNAUTHORIZED, self.to_string()),
            Self::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            Self::Kafka(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::RDKafka(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Self::SerdeJson(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Self::CanceledMessage(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };

        eprintln!("Error encountered: {}", msg); // Replace with a proper logging mechanism in production

        (status, msg).into_response()
    }
}
