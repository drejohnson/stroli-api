use crate::domain::secrets::AppSecrets;
use surrealdb::engine::any;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use surrealdb::Error as SurrealError;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database connection error: {0}")]
    ConnectionError(SurrealError),
    #[error("Record not found")]
    RecordNotFound,
    #[error("Database query error: {0}")]
    QueryError(String),
}

impl From<SurrealError> for Error {
    fn from(err: SurrealError) -> Self {
        Error::ConnectionError(err)
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

pub type DB = Surreal<Any>;

pub async fn connect_to_database(secrets: &AppSecrets) -> Result<Surreal<Any>> {
    let db = any::connect(&secrets.db_endpoint).await?;
    db.use_ns(&secrets.db_namespace)
        .use_db(&secrets.db_database_name)
        .await?;
    db.signin(Root {
        username: &secrets.db_username,
        password: &secrets.db_password,
    })
    .await?;

    Ok(db)
}
