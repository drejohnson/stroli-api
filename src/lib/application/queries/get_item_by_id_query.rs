use axum::extract::{Path, State};
use axum::{response::IntoResponse, Json};
use std::sync::{Arc, RwLock};

use crate::domain::repositories::ItemRepository;
use crate::domain::AppState;
use crate::errors::ApiError;

pub async fn get_item_by_id_query(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let repository = ItemRepository::new();

    repository
        .get_by_id(state.clone(), &id)
        .await
        .map(|item| Json(item))
        .map_err(|_| ApiError::InternalServerError)
}
