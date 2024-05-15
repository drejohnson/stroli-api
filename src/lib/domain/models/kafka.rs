use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMessage {
    pub name: String,
    pub message: String,
}

impl<'a> CustomMessage {
    pub fn name(&'a self) -> &'a str {
        &self.name
    }

    pub fn message(&'a self) -> &'a str {
        &self.message
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KafkaMessage {
    pub action: Action,
    pub message_id: i32,
    pub data: Option<CustomMessage>,
}

impl<'a> KafkaMessage {
    pub fn data(&'a self) -> &'a CustomMessage {
        self.data.as_ref().unwrap()
    }

    pub fn message_id(&'a self) -> &'a i32 {
        &self.message_id
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Create,
    Update,
    Delete,
}
