# Phase 4, Project 1: Message Queue Consumer

## Introduction

In modern distributed systems, components rarely communicate by calling each other
directly. Instead, they exchange **events** through a message queue. A producer
publishes an event ("task was created") and one or more consumers pick it up and
react (send a notification, update a search index, trigger a workflow).

This decoupling brings several advantages:

- **Resilience** -- if the consumer is down, messages queue up and are processed
  when it recovers.
- **Scalability** -- you can add more consumer instances to keep up with load.
- **Flexibility** -- new consumers can subscribe without changing the producer.

In this project you will build an **event-driven notification service** that
consumes task-lifecycle events from a message queue and dispatches them to
pluggable notification handlers. You will implement two queue backends -- an
in-memory backend for testing and a Redis Streams backend for production -- and
wire them behind a common trait so they can be swapped transparently.

---

## Part 1: Event Types

Define the events your system understands.

Create an `Event` enum with three variants:

| Variant        | Fields                                             |
|----------------|----------------------------------------------------|
| `TaskCreated`  | `task_id: Uuid`, `title: String`, `project_id: Uuid` |
| `TaskUpdated`  | `task_id: Uuid`, `field: String`, `old_value: String`, `new_value: String` |
| `TaskDeleted`  | `task_id: Uuid`                                    |

Use `#[serde(tag = "type")]` so the JSON representation includes a `"type"`
discriminator field. This makes it easy to route events by type on the consumer
side.

**Goal:** All event types serialize to JSON and deserialize back without loss.

---

## Part 2: Message Backend Trait

Define a `MessageBackend` trait that abstracts the queue:

```rust
#[async_trait]
pub trait MessageBackend: Send + Sync {
    async fn publish(&self, stream: &str, event: &Event) -> Result<String, BackendError>;
    async fn subscribe(&self, stream: &str, group: &str, consumer: &str)
        -> Result<Vec<(String, Event)>, BackendError>;
    async fn acknowledge(&self, stream: &str, group: &str, id: &str)
        -> Result<(), BackendError>;
    async fn dead_letter(&self, stream: &str, id: &str, event: &Event, error: &str)
        -> Result<(), BackendError>;
}
```

- `publish` -- add an event to the named stream and return its message ID.
- `subscribe` -- read the next batch of unacknowledged messages for the given
  consumer group / consumer name.
- `acknowledge` -- mark a message as successfully processed.
- `dead_letter` -- move a poison message to a dedicated dead-letter stream so it
  stops blocking the consumer.

**Goal:** Any struct that implements `MessageBackend` can be used by the consumer
worker without code changes.

---

## Part 3: In-Memory Backend

Implement `MessageBackend` for an `InMemoryBackend` struct backed by
`tokio::sync` channels and internal `Arc<Mutex<...>>` state.

This backend is not production-grade -- it exists so you can write fast,
deterministic tests without a running Redis instance.

Key behaviors to implement:

- Messages are stored in an append-only `Vec` per stream.
- `subscribe` returns messages that have not yet been acknowledged by the
  requesting consumer group.
- `acknowledge` marks a message as consumed for that group.
- `dead_letter` appends the message (plus the error reason) to a
  `"{stream}:dead"` stream.

**Goal:** The full test suite passes using only `InMemoryBackend`.

---

## Part 4: Redis Streams Backend

Implement `MessageBackend` for a `RedisBackend` struct.

Redis Streams provide a log-like data structure with consumer group support:

| Operation       | Redis Command                                     |
|-----------------|----------------------------------------------------|
| Publish         | `XADD stream * field value ...`                    |
| Create group    | `XGROUP CREATE stream group $ MKSTREAM`            |
| Read            | `XREADGROUP GROUP group consumer COUNT n BLOCK ms STREAMS stream >` |
| Acknowledge     | `XACK stream group id`                             |
| Dead letter     | `XADD stream:dead * ...`                           |

Wrap each command with proper error mapping to `BackendError`.

**Goal:** The consumer can run against a local Redis instance and process
messages published by any Redis client.

---

## Part 5: Consumer Worker

Build a `Consumer` struct that ties together a backend and a set of event
handlers:

```
loop {
    messages = backend.subscribe(stream, group, consumer_name)
    for (id, event) in messages {
        match handler.handle(event).await {
            Ok(()) => backend.acknowledge(stream, group, id),
            Err(e) => // retry logic (see Part 6)
        }
    }
}
```

The consumer should:

1. Poll the backend in a loop.
2. Deserialize each message into an `Event`.
3. Dispatch to the appropriate handler.
4. Acknowledge on success.
5. Shut down gracefully when a `tokio::sync::watch` or
   `tokio_util::CancellationToken` signal is received -- finish processing the
   current batch, then exit.

**Goal:** The consumer processes every published event exactly once under normal
conditions.

---

## Part 6: Retry and Dead Letter

Not every message can be processed on the first try. Implement:

- **Exponential backoff** -- on failure, wait 100ms, 200ms, 400ms, ... before
  retrying.  Use the `backoff` crate's `ExponentialBackoff`.
- **Max retries** -- after 3 failed attempts, stop retrying.
- **Dead letter** -- call `backend.dead_letter(...)` with the original event and
  the last error message so operators can inspect and replay later.

**Goal:** Transient failures are retried automatically; poison messages do not
block the queue forever.

---

## Part 7: Notification Handlers

Define an `EventHandler` trait:

```rust
#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle(&self, event: &Event) -> Result<(), ConsumerError>;
}
```

Implement at least three handlers:

| Handler          | Behavior                                          |
|------------------|---------------------------------------------------|
| `LogHandler`     | Logs the event using `tracing::info!`             |
| `WebhookHandler` | POSTs the serialized event to a configured URL (stub) |
| `EmailHandler`   | Sends an email notification (stub)                |

The consumer should support registering multiple handlers so a single event can
trigger logging **and** a webhook, for example.

**Goal:** Handlers are composable and independently testable.
