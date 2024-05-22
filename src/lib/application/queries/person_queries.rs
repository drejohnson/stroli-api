use axum::extract::{Path, State};
use axum::{response::IntoResponse, Json};

use crate::domain::models::person::{Person, PersonResponse};
use crate::domain::repositories::person_repository::{Error, PersonRepository, Result};
use crate::domain::state::SharedState;

pub async fn get_by_id_query(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let repository = PersonRepository::new(state.db.clone());

    match repository.get_by_id(&id).await {
        Ok(Some(person)) => Ok(Json(PersonResponse {
            id: person.get_id().ok_or(Error::RecordNotFound)?,
            name: person.name,
        })),
        Ok(None) => Err(Error::RecordNotFound),
        Err(e) => Err(e),
    }
}

pub async fn get_all_query(State(state): State<SharedState>) -> Result<impl IntoResponse> {
    let repository = PersonRepository::new(state.db.clone());

    let result = repository.get_all().await?;

    let persons: Vec<PersonResponse> = result
        .into_iter()
        .map(|p: Person| PersonResponse {
            id: p.get_id().unwrap_or_else(|| "No ID".to_string()),
            name: p.name,
        })
        .collect();

    Ok(Json(persons))
}
