use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::backend::MessageBackend;
use crate::error::BackendError;
use crate::events::Event;

/// A stored message inside the in-memory backend.
#[derive(Debug, Clone)]
struct StoredMessage {
    id: String,
    event: Event,
}

/// Internal state shared across clones of `InMemoryBackend`.
#[derive(Debug, Default)]
struct State {
    /// stream_name -> ordered list of messages
    streams: HashMap<String, Vec<StoredMessage>>,
    /// (stream, group) -> set of acknowledged message IDs
    acknowledged: HashMap<(String, String), HashSet<String>>,
    /// Auto-incrementing counter used to generate message IDs.
    next_id: u64,
}

/// An in-memory `MessageBackend` implementation backed by `Arc<Mutex<...>>`.
///
/// Suitable for unit and integration tests -- not for production use.
#[derive(Debug, Clone, Default)]
pub struct InMemoryBackend {
    state: Arc<Mutex<State>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self::default()
    }

    /// Return all messages currently in the dead-letter stream for `stream`.
    pub async fn dead_letters(&self, stream: &str) -> Vec<(String, Event)> {
        todo!()
    }
}

#[async_trait]
impl MessageBackend for InMemoryBackend {
    async fn publish(&self, stream: &str, event: &Event) -> Result<String, BackendError> {
        todo!()
    }

    async fn subscribe(
        &self,
        stream: &str,
        group: &str,
        consumer: &str,
    ) -> Result<Vec<(String, Event)>, BackendError> {
        todo!()
    }

    async fn acknowledge(
        &self,
        stream: &str,
        group: &str,
        id: &str,
    ) -> Result<(), BackendError> {
        todo!()
    }

    async fn dead_letter(
        &self,
        stream: &str,
        id: &str,
        event: &Event,
        error: &str,
    ) -> Result<(), BackendError> {
        todo!()
    }
}
