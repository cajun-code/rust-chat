use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WebSocketMessageType {
    NewMessage,
    UserList,
    UserNameChange,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WebSocketMessage {
    pub message_type: WebSocketMessageType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}


#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatMessage {
    pub message: String,
    pub author: String,
    pub created_at: NaiveDateTime,
}