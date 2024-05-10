use std::sync::{Arc, RwLock};

use crate::domain::app_secrets::AppSecrets;
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: edgedb_tokio::Client,
    pub secrets: AppSecrets,
    pub key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl AppState {
    pub fn initialize(db: edgedb_tokio::Client, secrets: AppSecrets) -> Self {
        Self {
            db,
            secrets,
            key: Key::generate(),
        }
    }

    pub fn initialize_shared_state(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}
