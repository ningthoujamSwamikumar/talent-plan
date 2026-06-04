pub mod backend;
pub mod consumer;
pub mod error;
pub mod events;
pub mod handler;
pub mod memory_backend;
pub mod redis_backend;

pub use backend::MessageBackend;
pub use consumer::{Consumer, ConsumerConfig};
pub use error::{BackendError, ConsumerError};
pub use events::{Event, EventEnvelope};
pub use handler::{
    CompositeHandler, EmailHandler, EventHandler, LogHandler, WebhookHandler,
};
pub use memory_backend::InMemoryBackend;
pub use redis_backend::RedisBackend;
