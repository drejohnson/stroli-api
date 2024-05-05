use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemEntry {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: i32,
}
