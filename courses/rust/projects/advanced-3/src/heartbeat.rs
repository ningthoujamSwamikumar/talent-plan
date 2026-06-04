use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::manager::ConnectionManager;
use crate::message::ServerMessage;

/// Configuration for the heartbeat system.
#[derive(Debug, Clone)]
pub struct HeartbeatConfig {
    /// Interval between pings sent to clients.
    pub ping_interval: Duration,
    /// Maximum time to wait for a pong before considering the client dead.
    pub pong_timeout: Duration,
}

impl Default for HeartbeatConfig {
    fn default() -> Self {
        Self {
            ping_interval: Duration::from_secs(30),
            pong_timeout: Duration::from_secs(10),
        }
    }
}

/// Spawn a heartbeat task for a single connection.
///
/// Sends `ServerMessage::Ping` at the configured interval through the provided
/// sender. If no pong is recorded within the timeout, signals disconnect through
/// the `disconnect_tx` channel.
///
/// The caller should notify pong receipt by sending through `pong_rx`.
pub fn spawn_heartbeat(
    _user_id: String,
    _sender: mpsc::Sender<ServerMessage>,
    _pong_rx: mpsc::Receiver<()>,
    _disconnect_tx: mpsc::Sender<String>,
    _config: HeartbeatConfig,
) -> tokio::task::JoinHandle<()> {
    todo!()
}

/// Spawn a global monitor that listens for disconnect signals and cleans up
/// rooms via the `ConnectionManager`.
pub fn spawn_disconnect_monitor(
    _manager: Arc<ConnectionManager>,
    _disconnect_rx: mpsc::Receiver<String>,
) -> tokio::task::JoinHandle<()> {
    todo!()
}
