use axum::{
    extract::{Path, State},
    Json,
};
use serde_json::json;
use std::sync::{Arc, RwLock};

use crate::domain::repositories::ItemRepository;
use crate::domain::AppState;
use crate::errors::ApiError;

pub async fn delete_item_command(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<String>, // Extracting the ID from the path
) -> Result<Json<serde_json::Value>, ApiError> {
    let repository = ItemRepository::new();

    repository.delete_item(state.clone(), &id).await.map(|_| {
        Json(json!({
            "status": "success",
            "message": "Item deleted successfully."
        }))
    })
}
