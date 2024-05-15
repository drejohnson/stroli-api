use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde_json::json;
use std::sync::{Arc, RwLock};

use crate::domain::models::item::Item;
use crate::domain::AppState;
use crate::{domain::repositories::ItemRepository, errors::ApiError};

pub async fn create_item_command(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(body): Json<Item>,
) -> Result<impl IntoResponse, ApiError> {
    let repository = ItemRepository::new();

    // Check if an item with the same name already exists
    if let Ok(_) = repository
        .get_by_name(state.clone(), body.name.clone())
        .await
    {
        // If an item exists, return an error using ApiError
        return Err(ApiError::BadRequest(format!(
            "Item with name {} already exists",
            body.name
        )));
    }

    // Proceed to create a new item
    repository
        .create_item(state, body)
        .await
        .map(|created_item| Json(json!({"status": "success", "data": created_item})))
        .map_err(|_| ApiError::InternalServerError)
}
