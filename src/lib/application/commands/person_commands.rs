use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
};
use http::StatusCode;

use crate::domain::repositories::person_repository::{Error, PersonRepository, Result};
use crate::domain::{
    models::person::{Person, PersonResponse},
    state::SharedState,
};

pub async fn create_command(
    State(state): State<SharedState>,
    Json(person): Json<Person>,
) -> Result<impl IntoResponse> {
    // TODO: Remove and use the repository from the extension
    let repository = PersonRepository::new(state.db.clone());

    // Attempt to create the new person
    match repository.create_person(person).await {
        Ok(created_persons) => match created_persons.get(0) {
            Some(person) => {
                let response = PersonResponse {
                    id: person.get_id().ok_or(Error::RecordNotFound)?,
                    name: person.name.clone(),
                };
                Ok(Json(response))
            }
            None => Err(Error::RecordNotFound),
        },
        Err(e) => Err(e),
    }
}

pub async fn update_command(
    State(state): State<SharedState>,
    Path(id): Path<String>,
    Json(person): Json<Person>,
) -> Result<impl IntoResponse> {
    let repository = PersonRepository::new(state.db.clone());

    match repository.update_person(&id, person).await {
        Ok(Some(updated_person)) => Ok(Json(PersonResponse {
            id: updated_person.get_id().ok_or(Error::RecordNotFound)?,
            name: updated_person.name,
        })),
        Ok(None) => Err(Error::RecordNotFound),
        Err(e) => Err(e),
    }
}

pub async fn delete_command(
    State(state): State<SharedState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse> {
    let repository = PersonRepository::new(state.db.clone());

    match repository.delete_person(&id).await {
        Ok(Some(_)) => Ok(StatusCode::NO_CONTENT),
        Ok(None) => Err(Error::RecordNotFound),
        Err(e) => Err(e),
    }
}
