use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;
use tracing::error;

use crate::domain::{
    models::person::Person,
    state::{self, AppState},
};

pub trait DbBmc {
    const TABLE: &'static str;
}

pub struct PersonRepository {
    db: Surreal<Any>,
    table: &'static str,
}

impl DbBmc for PersonRepository {
    const TABLE: &'static str = "person";
}

impl PersonRepository {
    pub fn new(db: Surreal<Any>) -> Self {
        PersonRepository {
            db,
            table: "person",
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Person>> {
        self.db.select(self.table).await.map_err(Error::from)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Option<Person>> {
        self.db.select((self.table, id)).await.map_err(Error::from)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<Option<Vec<Person>>> {
        self.db
            .select((self.table, name))
            .await
            .map_err(Error::from)
    }

    pub async fn create<MC, E>(state: &AppState, data: E) -> Result<Vec<E>>
    where
        MC: DbBmc,
        for<'a> E: Serialize + Deserialize<'a>,
    {
        let db = &state.db;
        let entity = db.create(MC::TABLE).content(data).await?;
        Ok(entity)
    }

    pub async fn create_person(&self, content: Person) -> Result<Vec<Person>> {
        let result: Vec<Person> = self
            .db
            .create(self.table)
            .content(content)
            .await
            .map_err(Error::from)?;

        Ok(result) // Return the vector directly
    }

    pub async fn update_person(&self, id: &str, content: Person) -> Result<Option<Person>> {
        self.db
            .update((self.table, id))
            .content(content)
            .await
            .map_err(Error::from)
    }

    pub async fn delete_person(&self, id: &str) -> Result<Option<Person>> {
        self.db.delete((self.table, id)).await.map_err(Error::from)
    }
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

impl From<surrealdb::Error> for Error {
    fn from(err: surrealdb::Error) -> Self {
        Error::QueryError(err.to_string())
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
