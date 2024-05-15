use axum::extract::State;
use axum::{response::IntoResponse, Json};
use std::sync::{Arc, RwLock};

use crate::domain::repositories::ItemRepository;
use crate::domain::AppState;
use crate::errors::ApiError;

pub async fn get_all_items_query(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<impl IntoResponse, ApiError> {
    let repository = ItemRepository::new();

    repository
        .get_all(state.clone())
        .await
        .map(|items| Json(items))
        .map_err(|_| ApiError::InternalServerError)
}
