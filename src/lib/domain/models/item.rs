use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
}
