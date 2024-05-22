use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use axum_macros::debug_handler;
use http::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use surrealdb::sql::Thing;
use tracing::debug;
use tracing::error;

use crate::domain::state::SharedState;

const PERSON: &str = "person";

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: Option<Thing>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonResponse {
    pub id: String,
    pub name: String,
}

impl Person {
    fn get_id(&self) -> Option<String> {
        self.id.as_ref().map(|t| t.id.to_string())
    }
}

#[debug_handler]
pub async fn create(
    State(state): State<SharedState>,
    Json(person): Json<Person>,
) -> Result<Json<PersonResponse>> {
    let result: Vec<Person> = state
        .db
        .create(PERSON)
        .content(person)
        .await
        .map_err(|err| Error::QueryError(err.to_string()))?;

    match result.get(0) {
        Some(person) => {
            let response = PersonResponse {
                id: person.get_id().ok_or(Error::RecordNotFound)?,
                name: person.name.clone(),
            };
            debug!("{:?}", &response);
            Ok(Json(response))
        }
        None => Err(Error::RecordNotFound),
    }
}

pub async fn read(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<Json<PersonResponse>> {
    let person: Option<Person> = state
        .db
        .select((PERSON, &*id))
        .await
        .map_err(|err| Error::QueryError(err.to_string()))?;

    match person {
        Some(person) => Ok(Json(PersonResponse {
            id: person.get_id().ok_or(Error::RecordNotFound)?,
            name: person.name,
        })),
        None => Err(Error::RecordNotFound),
    }
}

pub async fn update(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(person): Json<Person>,
) -> Result<Json<PersonResponse>> {
    let updated_person: Option<Person> = state
        .db
        .update((PERSON, &*id))
        .content(person)
        .await
        .map_err(|err| Error::QueryError(err.to_string()))?;

    match updated_person {
        Some(person) => Ok(Json(PersonResponse {
            id: person.get_id().ok_or(Error::RecordNotFound)?,
            name: person.name,
        })),
        None => Err(Error::RecordNotFound),
    }
}

pub async fn delete(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let _: Option<Person> = state
        .db
        .delete((PERSON, &*id))
        .await
        .map_err(|err| Error::QueryError(err.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list(State(state): State<SharedState>) -> Result<Json<Vec<PersonResponse>>> {
    let result = state
        .db
        .select(PERSON)
        .await
        .map_err(|err| Error::QueryError(err.to_string()))?;

    let people: Vec<PersonResponse> = result
        .into_iter()
        .map(|p: Person| PersonResponse {
            id: p.get_id().unwrap_or_else(|| "No ID".to_string()),
            name: p.name,
        })
        .collect();

    Ok(Json(people))
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Record not found")]
    RecordNotFound,
    #[error("Database query error: {0}")]
    QueryError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("{:<12} - query::Error {:?}", "INTO_RES", &self);

        let (status, error_message) = match &self {
            Error::RecordNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            Error::QueryError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
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
