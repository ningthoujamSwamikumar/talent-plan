use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::watch;
use uuid::Uuid;

use event_consumer::{
    BackendError, Consumer, ConsumerConfig, ConsumerError, Event, EventHandler,
    InMemoryBackend, MessageBackend,
};

// ---------------------------------------------------------------------------
// Test helpers
// ---------------------------------------------------------------------------

/// A handler that records every event it receives.
#[derive(Default)]
struct RecordingHandler {
    events: tokio::sync::Mutex<Vec<Event>>,
}

#[async_trait]
impl EventHandler for RecordingHandler {
    async fn handle(&self, event: &Event) -> Result<(), ConsumerError> {
        self.events.lock().await.push(event.clone());
        Ok(())
    }
}

/// A handler that always fails.
struct FailingHandler {
    attempts: AtomicU32,
}

impl FailingHandler {
    fn new() -> Self {
        Self {
            attempts: AtomicU32::new(0),
        }
    }

    fn attempt_count(&self) -> u32 {
        self.attempts.load(Ordering::SeqCst)
    }
}

#[async_trait]
impl EventHandler for FailingHandler {
    async fn handle(&self, _event: &Event) -> Result<(), ConsumerError> {
        self.attempts.fetch_add(1, Ordering::SeqCst);
        Err(ConsumerError::Handler("always fails".into()))
    }
}

/// A handler that fails N times then succeeds.
struct FailNTimesHandler {
    fail_count: u32,
    attempts: AtomicU32,
}

impl FailNTimesHandler {
    fn new(fail_count: u32) -> Self {
        Self {
            fail_count,
            attempts: AtomicU32::new(0),
        }
    }
}

#[async_trait]
impl EventHandler for FailNTimesHandler {
    async fn handle(&self, _event: &Event) -> Result<(), ConsumerError> {
        let n = self.attempts.fetch_add(1, Ordering::SeqCst);
        if n < self.fail_count {
            Err(ConsumerError::Handler(format!("failure #{}", n + 1)))
        } else {
            Ok(())
        }
    }
}

fn sample_task_created() -> Event {
    Event::TaskCreated {
        task_id: Uuid::new_v4(),
        title: "Write tests".into(),
        project_id: Uuid::new_v4(),
    }
}

fn sample_task_updated() -> Event {
    Event::TaskUpdated {
        task_id: Uuid::new_v4(),
        field: "status".into(),
        old_value: "open".into(),
        new_value: "in_progress".into(),
    }
}

fn sample_task_deleted() -> Event {
    Event::TaskDeleted {
        task_id: Uuid::new_v4(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Publishing an event and then subscribing should return that event.
#[tokio::test]
async fn publish_and_consume_delivers_message() {
    let backend = InMemoryBackend::new();
    let event = sample_task_created();

    let id = backend.publish("tasks", &event).await.unwrap();
    assert!(!id.is_empty());

    let messages = backend.subscribe("tasks", "grp", "c1").await.unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].0, id);
}

/// After acknowledging a message it should not be redelivered.
#[tokio::test]
async fn consumer_acknowledges_processed_messages() {
    let backend = InMemoryBackend::new();
    let event = sample_task_created();

    let id = backend.publish("tasks", &event).await.unwrap();
    let _ = backend.subscribe("tasks", "grp", "c1").await.unwrap();
    backend.acknowledge("tasks", "grp", &id).await.unwrap();

    let messages = backend.subscribe("tasks", "grp", "c1").await.unwrap();
    assert!(messages.is_empty());
}

/// Messages that were delivered but not acknowledged should be returned again
/// on the next subscribe call.
#[tokio::test]
async fn unacknowledged_messages_redelivered() {
    let backend = InMemoryBackend::new();
    let event = sample_task_created();

    let _id = backend.publish("tasks", &event).await.unwrap();

    let first = backend.subscribe("tasks", "grp", "c1").await.unwrap();
    assert_eq!(first.len(), 1);

    // Do NOT acknowledge -- the message should appear again.
    let second = backend.subscribe("tasks", "grp", "c1").await.unwrap();
    assert_eq!(second.len(), 1);
    assert_eq!(first[0].0, second[0].0);
}

/// After 3 consecutive handler failures the message should land in the
/// dead-letter stream.
#[tokio::test]
async fn dead_letter_after_max_retries() {
    let backend = Arc::new(InMemoryBackend::new());
    let handler = Arc::new(FailingHandler::new());

    let event = sample_task_created();
    backend.publish("tasks", &event).await.unwrap();

    let config = ConsumerConfig::new("tasks", "grp", "c1").with_max_retries(3);
    let (_tx, rx) = watch::channel(false);
    let consumer = Consumer::new(backend.clone(), handler.clone(), config, rx);

    // Run should process the one message (with retries) then find no more.
    // We set up a shutdown signal after a short delay.
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let consumer = Consumer::new(backend.clone(), handler.clone(),
        ConsumerConfig::new("tasks", "grp", "c1").with_max_retries(3),
        shutdown_rx,
    );

    let be = backend.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        let _ = shutdown_tx.send(true);
    });

    let _ = consumer.run().await;

    let dead = be.dead_letters("tasks").await;
    assert!(!dead.is_empty(), "message should be in dead-letter stream");
}

/// Two consumers in the same group should each receive disjoint messages --
/// no message is processed more than once.
#[tokio::test]
async fn multiple_consumers_compete() {
    let backend = InMemoryBackend::new();

    for _ in 0..10 {
        backend.publish("tasks", &sample_task_created()).await.unwrap();
    }

    let mut seen_c1 = Vec::new();
    let mut seen_c2 = Vec::new();

    // Each consumer takes a turn.
    loop {
        let batch = backend.subscribe("tasks", "grp", "c1").await.unwrap();
        if batch.is_empty() {
            break;
        }
        for (id, _event) in &batch {
            backend.acknowledge("tasks", "grp", id).await.unwrap();
            seen_c1.push(id.clone());
        }

        let batch = backend.subscribe("tasks", "grp", "c2").await.unwrap();
        if batch.is_empty() {
            break;
        }
        for (id, _event) in &batch {
            backend.acknowledge("tasks", "grp", id).await.unwrap();
            seen_c2.push(id.clone());
        }
    }

    let total = seen_c1.len() + seen_c2.len();
    assert_eq!(total, 10, "all messages should be processed exactly once");

    // No duplicates.
    let mut all = seen_c1.clone();
    all.extend(seen_c2.clone());
    all.sort();
    all.dedup();
    assert_eq!(all.len(), 10);
}

/// TaskCreated round-trips through JSON correctly.
#[tokio::test]
async fn event_serialization_roundtrip_task_created() {
    let event = sample_task_created();
    let json = serde_json::to_string(&event).unwrap();
    let parsed: Event = serde_json::from_str(&json).unwrap();
    assert_eq!(event.task_id(), parsed.task_id());
}

/// TaskUpdated round-trips through JSON correctly.
#[tokio::test]
async fn event_serialization_roundtrip_task_updated() {
    let event = sample_task_updated();
    let json = serde_json::to_string(&event).unwrap();
    let parsed: Event = serde_json::from_str(&json).unwrap();
    assert_eq!(event.task_id(), parsed.task_id());
}

/// TaskDeleted round-trips through JSON correctly.
#[tokio::test]
async fn event_serialization_roundtrip_task_deleted() {
    let event = sample_task_deleted();
    let json = serde_json::to_string(&event).unwrap();
    let parsed: Event = serde_json::from_str(&json).unwrap();
    assert_eq!(event.task_id(), parsed.task_id());
}

/// The consumer should finish processing its current batch and then exit
/// cleanly when a shutdown signal is received.
#[tokio::test]
async fn consumer_graceful_shutdown() {
    let backend = Arc::new(InMemoryBackend::new());
    let handler = Arc::new(RecordingHandler::default());

    backend.publish("tasks", &sample_task_created()).await.unwrap();

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let config = ConsumerConfig::new("tasks", "grp", "c1");
    let consumer = Consumer::new(backend.clone(), handler.clone(), config, shutdown_rx);

    // Signal shutdown almost immediately -- consumer should still process
    // the one pending message.
    let h = tokio::spawn(async move {
        consumer.run().await
    });

    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    shutdown_tx.send(true).unwrap();

    let result = h.await.unwrap();
    assert!(result.is_ok());

    let events = handler.events.lock().await;
    assert_eq!(events.len(), 1, "the pending message should have been processed before shutdown");
}

/// Exponential backoff should increase the delay between retry attempts.
#[tokio::test]
async fn backoff_increases_between_retries() {
    // This test is a design-level check: we verify the handler is invoked
    // the expected number of times (max_retries) before the message is
    // dead-lettered. Actual timing verification is left to the student as a
    // stretch goal.
    let backend = Arc::new(InMemoryBackend::new());
    let handler = Arc::new(FailingHandler::new());

    backend.publish("tasks", &sample_task_created()).await.unwrap();

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let config = ConsumerConfig::new("tasks", "grp", "c1").with_max_retries(3);
    let consumer = Consumer::new(backend.clone(), handler.clone(), config, shutdown_rx);

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        let _ = shutdown_tx.send(true);
    });

    let _ = consumer.run().await;

    assert!(
        handler.attempt_count() >= 3,
        "handler should have been attempted at least max_retries times"
    );
}

/// Subscribing to an empty stream should return an empty vec, not an error.
#[tokio::test]
async fn empty_stream_returns_empty() {
    let backend = InMemoryBackend::new();
    let messages = backend.subscribe("nonexistent", "grp", "c1").await.unwrap();
    assert!(messages.is_empty());
}

/// Messages published to different streams should not interfere.
#[tokio::test]
async fn publish_to_different_streams_isolated() {
    let backend = InMemoryBackend::new();

    backend.publish("stream-a", &sample_task_created()).await.unwrap();
    backend.publish("stream-b", &sample_task_deleted()).await.unwrap();

    let a = backend.subscribe("stream-a", "grp", "c1").await.unwrap();
    let b = backend.subscribe("stream-b", "grp", "c1").await.unwrap();

    assert_eq!(a.len(), 1);
    assert_eq!(b.len(), 1);

    // Verify the right event ended up in the right stream.
    assert!(matches!(a[0].1, Event::TaskCreated { .. }));
    assert!(matches!(b[0].1, Event::TaskDeleted { .. }));
}

/// The handler should receive the correct event type for each message.
#[tokio::test]
async fn handler_receives_correct_event_type() {
    let backend = Arc::new(InMemoryBackend::new());
    let handler = Arc::new(RecordingHandler::default());

    let created = sample_task_created();
    let updated = sample_task_updated();
    let deleted = sample_task_deleted();

    backend.publish("tasks", &created).await.unwrap();
    backend.publish("tasks", &updated).await.unwrap();
    backend.publish("tasks", &deleted).await.unwrap();

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let config = ConsumerConfig::new("tasks", "grp", "c1");
    let consumer = Consumer::new(backend.clone(), handler.clone(), config, shutdown_rx);

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        let _ = shutdown_tx.send(true);
    });

    let _ = consumer.run().await;

    let events = handler.events.lock().await;
    assert_eq!(events.len(), 3);
    assert!(matches!(events[0], Event::TaskCreated { .. }));
    assert!(matches!(events[1], Event::TaskUpdated { .. }));
    assert!(matches!(events[2], Event::TaskDeleted { .. }));
}

/// If a handler returns an error, the consumer should not crash -- it should
/// continue processing subsequent messages (after retry / dead-letter logic).
#[tokio::test]
async fn consumer_continues_after_handler_error() {
    let backend = Arc::new(InMemoryBackend::new());

    // Publish two events. The handler will fail on the first, succeed on the second.
    backend.publish("tasks", &sample_task_created()).await.unwrap();
    backend.publish("tasks", &sample_task_created()).await.unwrap();

    let handler = Arc::new(FailNTimesHandler::new(3));

    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let config = ConsumerConfig::new("tasks", "grp", "c1").with_max_retries(3);
    let consumer = Consumer::new(backend.clone(), handler.clone(), config, shutdown_rx);

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
        let _ = shutdown_tx.send(true);
    });

    let result = consumer.run().await;
    assert!(result.is_ok(), "consumer should not crash on handler errors");
}
