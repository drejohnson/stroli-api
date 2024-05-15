use std::sync::{Arc, RwLock};

use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use rdkafka::producer::FutureProducer;
use shuttle_runtime::{DeploymentMetadata, Environment};
use surrealdb::{engine::any::Any, Surreal};

use crate::domain::AppSecrets;
use crate::infrastructure;

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Clone)]
pub struct AppState {
    pub db: Surreal<Any>,
    pub kafka_producer: FutureProducer,
    pub secrets: AppSecrets,
    pub key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}

impl AppState {
    pub fn initialize(
        db: Surreal<Any>,
        secrets: AppSecrets,
        metadata: &DeploymentMetadata,
    ) -> Self {
        let kafka_producer = match metadata.env {
            Environment::Local => infrastructure::kafka::producer::create_kafka_producer(&secrets),
            Environment::Deployment => {
                infrastructure::kafka::producer::create_kafka_producer_upstash(&secrets)
            }
        };
        Self {
            db,
            kafka_producer,
            secrets,
            key: Key::generate(),
        }
    }

    pub fn initialize_shared_state(self) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(self))
    }
}

impl<'a> AppState {
    pub fn producer(&'a self) -> &'a FutureProducer {
        &self.kafka_producer
    }
}
