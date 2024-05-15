use std::sync::{Arc, RwLock};

use crate::{
    domain::{AppState, Item},
    errors::ApiError,
};

pub struct ItemRepository {
    table: String,
}

impl ItemRepository {
    pub fn new() -> Self {
        ItemRepository {
            table: String::from("item"),
        }
    }

    pub async fn get_all(&self, app_state: Arc<RwLock<AppState>>) -> Result<Vec<Item>, ApiError> {
        let state = app_state.read().map_err(|_| ApiError::LockPoisoned)?;
        let records = state.db.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(
        &self,
        app_state: Arc<RwLock<AppState>>,
        id: &str,
    ) -> Result<Item, ApiError> {
        let state = app_state.read().map_err(|_| ApiError::LockPoisoned)?;
        if let Some(record) = state.db.select((&self.table, id)).await? {
            return Ok(record);
        }

        Err(ApiError::NotFound(format!("Item with id {} not found", id)))
    }

    pub async fn get_by_name(
        &self,
        app_state: Arc<RwLock<AppState>>,
        name: String,
    ) -> Result<Item, ApiError> {
        let state = app_state.read().map_err(|_| ApiError::LockPoisoned)?;
        if let Some(record) = state
            .db
            .query(format!("SELECT * FROM {} WHERE name = $name", &self.table))
            .bind(("name", name.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }

        Err(ApiError::NotFound(format!(
            "Item with name {} not found",
            name
        )))
    }

    pub async fn create_item(
        &self,
        app_state: Arc<RwLock<AppState>>,
        content: Item,
    ) -> Result<Vec<Item>, ApiError> {
        let state = app_state.read().map_err(|_| ApiError::LockPoisoned)?;
        let record = state.db.create(&self.table).content(content).await?;
        Ok(record)
    }

    pub async fn update_item(
        &self,
        app_state: Arc<RwLock<AppState>>,
        content: Item,
    ) -> Result<Item, ApiError> {
        // Handle potential lock poisoning and avoid panicking
        let state = app_state.read().map_err(|_| ApiError::LockPoisoned)?;

        // Match on the ID option to handle both cases
        match &content.id {
            Some(id) if !id.is_empty() => {
                // Perform the update using a reference to id
                match state.db.update((&self.table, id)).content(content).await {
                    Ok(Some(record)) => Ok(record),
                    Ok(None) => Err(ApiError::NotFound("No item found to update.".to_string())),
                    Err(e) => Err(ApiError::Database(e)),
                }
            }
            _ => Err(ApiError::BadRequest(
                "Item ID is required for an update.".to_string(),
            )),
        }
    }

    pub async fn delete_item(
        &self,
        app_state: Arc<RwLock<AppState>>,
        id: &str,
    ) -> Result<Item, ApiError> {
        let state = app_state.read().map_err(|_| ApiError::LockPoisoned)?;
        if let Some(record) = state.db.delete((&self.table, id)).await? {
            return Ok(record);
        }

        Err(ApiError::NotFound(format!("Item with id {} not found", id)))
    }
}
