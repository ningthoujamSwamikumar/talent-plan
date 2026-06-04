use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use tokio::sync::mpsc;

use crate::manager::ConnectionManager;
use crate::message::{ClientMessage, ServerMessage};

/// Shared application state passed to handlers via axum's State extractor.
#[derive(Debug, Clone)]
pub struct AppState {
    pub manager: Arc<ConnectionManager>,
    /// Channel for signaling that a user should be disconnected (heartbeat timeout).
    pub disconnect_tx: mpsc::Sender<String>,
}

/// HTTP handler that upgrades the connection to WebSocket.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, _state))
}

/// Handles a single WebSocket connection after upgrade.
///
/// 1. Assigns a unique user_id.
/// 2. Splits the socket into sender/receiver.
/// 3. Creates a bounded channel for outgoing messages.
/// 4. Spawns a heartbeat task.
/// 5. Forwards incoming `ClientMessage`s to the `ConnectionManager`.
/// 6. Cleans up on disconnect.
async fn handle_socket(_socket: WebSocket, _state: AppState) {
    todo!()
}

/// Process a single incoming text frame, dispatching to the appropriate manager
/// method.
async fn process_message(
    _msg: &str,
    _user_id: &str,
    _state: &AppState,
    _pong_tx: &mpsc::Sender<()>,
) {
    todo!()
}
