use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: Option<Thing>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonResponse {
    pub id: String,
    pub name: String,
}

impl Person {
    pub fn get_id(&self) -> Option<String> {
        self.id.as_ref().map(|t| t.id.to_string())
    }
}
