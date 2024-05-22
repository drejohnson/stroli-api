use crate::application::commands::person_commands;
use crate::application::queries::person_queries;
// use crate::domain::person;
use crate::domain::state::SharedState;
use axum::response::Response;
use axum::Json;
use axum::{http::StatusCode, response::IntoResponse};
use axum::{
    http::{self, HeaderValue, Method},
    routing::{delete, get, post, put},
    Router,
};
use axum_macros::debug_handler;

use serde::Serialize;
use tower_http::{
    cors::CorsLayer,
    trace::{self, TraceLayer},
};
use tracing::{error, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// health_check endpoint handler
#[debug_handler]
pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub fn build_api_router(app_state: SharedState) -> Result<Router> {
    let domain = app_state.secrets.domain.clone();
    let parsed_domain = domain
        .parse::<HeaderValue>()
        .map_err(|e| Error::ParsingError(format!("Failed to parse domain {}: {}", domain, e)))?;
    let cors = CorsLayer::new()
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![
            http::header::ORIGIN,
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
        ])
        .allow_origin(parsed_domain);

    Ok(Router::new()
        .route("/health_check", get(health_check))
        //curl -X POST -H "Content-Type: application/json" -d '{"name":"John Doe"}' http://localhost:8000/api/person
        .route("/api/person", post(person_commands::create_command))
        //curl -X GET http://localhost:8000/api/person/klsei458igfj9ds
        .route("/api/person/:id", get(person_queries::get_by_id_query))
        //curl -X PUT -H "Content-Type: application/json" -d '{"name":"Jane Doe"}' http://localhost:8000/api/person/klsei458igfj9ds
        .route("/api/person/:id", put(person_commands::update_command))
        //curl -X DELETE http://localhost:8000/api/person/1
        .route("/api/person/:id", delete(person_commands::delete_command))
        //curl -X GET http://localhost:8000/api/people
        .route("/api/people", get(person_queries::get_all_query))
        .layer(cors)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(app_state))
}

// initialize tracing
pub fn initialize_tracing() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Domain parsinf error: {0}")]
    ParsingError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{:<12} - query::Error {:?}", "INTO_RES", &self);

        let (status, error_message) = match &self {
            Error::ParsingError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
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
