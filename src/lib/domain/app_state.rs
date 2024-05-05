use std::sync::{Arc, RwLock};

use super::AppSecrets;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: edgedb_tokio::Client,
    pub secrets: AppSecrets,
}

impl AppState {
    pub fn initialize(db: edgedb_tokio::Client, secrets: AppSecrets) -> Self {
        Self { db, secrets }
    }

    pub fn initialize_shared_state(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
