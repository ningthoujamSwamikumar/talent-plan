use tokio::sync::mpsc;

use crate::message::ServerMessage;

/// A handle representing one connected WebSocket client.
#[derive(Debug, Clone)]
pub struct Connection {
    pub user_id: String,
    pub sender: mpsc::Sender<ServerMessage>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

impl Connection {
    /// Create a new connection with a bounded outgoing channel sender.
    pub fn new(user_id: String, sender: mpsc::Sender<ServerMessage>) -> Self {
        todo!()
    }

    /// Try to send a message to this connection.
    /// Returns `false` if the channel is closed (client disconnected).
    pub async fn send(&self, msg: ServerMessage) -> bool {
        todo!()
    }

    /// Try to send a message without awaiting. If the channel is full, drop the
    /// oldest message first (backpressure strategy).
    pub fn try_send_with_backpressure(&self, _msg: ServerMessage) {
        todo!()
    }
}
