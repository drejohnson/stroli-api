use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::{Arc, RwLock};

use crate::domain::repositories::ItemRepository;
use crate::domain::{AppState, Item};
use crate::errors::ApiError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodoRequest {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
}

pub async fn update_todo_command(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateTodoRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let repository = ItemRepository::new();

    match repository.get_by_id(state.clone(), &id).await {
        Ok(todo) => {
            let updated_item = Item {
                id: Some(id), // Assuming id comes from the URL and is part of the item
                name: if body.name.is_empty() {
                    todo.name
                } else {
                    body.name
                },
                description: if body.description.is_empty() {
                    todo.description
                } else {
                    body.description
                },
                price: if body.price > 0.0 {
                    body.price
                } else {
                    todo.price
                },
                stock: if body.stock > 0 {
                    body.stock
                } else {
                    todo.stock
                },
            };

            repository
                .update_item(state, updated_item)
                .await
                .map(|item_response| Json(json!({"status": "success", "data": item_response})))
                .map_err(|_| ApiError::InternalServerError)
        }
        Err(_) => Err(ApiError::NotFound(format!(
            "Todo with ID: {} not found",
            id
        ))),
    }
}
