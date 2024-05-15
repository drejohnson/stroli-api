use crate::domain::AppSecrets;
use crate::errors::ApiError;
use surrealdb::engine::any;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub type DB = Surreal<Any>;

pub async fn connect_to_database(secrets: &AppSecrets) -> Result<DB, ApiError> {
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
