use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};





#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    pub message: String,
    pub author: String,
    pub created_at: NaiveDateTime,
}