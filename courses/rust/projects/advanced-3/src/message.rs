use serde::{Deserialize, Serialize};

/// Messages sent from clients to the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Join { room_id: String },
    Leave { room_id: String },
    Message { room_id: String, content: String },
    Pong,
    Edit { room_id: String, task_id: String, field: String, value: String },
}

/// Messages sent from the server to clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Joined { room_id: String, members: Vec<String> },
    Left { room_id: String, user_id: String },
    Message { room_id: String, user_id: String, content: String, timestamp: String },
    Presence { room_id: String, members: Vec<String> },
    Ping,
    Error { message: String },
    EditAck { task_id: String, field: String, value: String, timestamp: String },
}
