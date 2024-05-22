use std::sync::Arc;

use aws_sdk_s3::Client as S3Client;
use surrealdb::{engine::any::Any, Surreal};

use crate::domain::secrets::AppSecrets;

pub type SharedState = Arc<AppState>;

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Any>,
    pub secrets: AppSecrets,
    pub s3: S3Client,
}

impl AppState {
    pub fn initialize(db: &Surreal<Any>, s3: &S3Client, secrets: &AppSecrets) -> Self {
        Self {
            db: db.to_owned(),
            secrets: secrets.to_owned(),
            s3: s3.to_owned(),
        }
    }

    pub fn initialize_shared_state(self) -> Arc<Self> {
        Arc::new(self)
    }
}
