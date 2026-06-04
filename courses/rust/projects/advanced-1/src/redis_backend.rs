use async_trait::async_trait;
use redis::aio::MultiplexedConnection;
use redis::Client;

use crate::backend::MessageBackend;
use crate::error::BackendError;
use crate::events::Event;

/// A `MessageBackend` implementation backed by Redis Streams.
///
/// Uses `XADD`, `XREADGROUP`, `XACK`, and consumer groups to provide
/// reliable, competing-consumer message delivery.
pub struct RedisBackend {
    _client: Client,
    _connection: MultiplexedConnection,
}

impl RedisBackend {
    /// Connect to Redis at the given URL (e.g. `redis://127.0.0.1/`).
    pub async fn connect(redis_url: &str) -> Result<Self, BackendError> {
        todo!()
    }

    /// Ensure a consumer group exists for the given stream, creating the
    /// stream if necessary.
    pub async fn ensure_group(&self, _stream: &str, _group: &str) -> Result<(), BackendError> {
        todo!()
    }
}

#[async_trait]
impl MessageBackend for RedisBackend {
    async fn publish(&self, _stream: &str, _event: &Event) -> Result<String, BackendError> {
        todo!()
    }

    async fn subscribe(
        &self,
        _stream: &str,
        _group: &str,
        _consumer: &str,
    ) -> Result<Vec<(String, Event)>, BackendError> {
        todo!()
    }

    async fn acknowledge(
        &self,
        _stream: &str,
        _group: &str,
        _id: &str,
    ) -> Result<(), BackendError> {
        todo!()
    }

    async fn dead_letter(
        &self,
        _stream: &str,
        _id: &str,
        _event: &Event,
        _error: &str,
    ) -> Result<(), BackendError> {
        todo!()
    }
}
