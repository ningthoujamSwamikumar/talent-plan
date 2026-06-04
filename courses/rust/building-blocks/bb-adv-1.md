# Building Block Adv-1: Message Queues and Event-Driven Architecture

**Prerequisites**: Phase 3 complete (Projects prod-1 through prod-4).

Before starting [Project: Message Queue Consumer](../projects/advanced-1/README.md),
complete the readings and exercises below.

## What to read

- [Redis Streams documentation](https://redis.io/docs/data-types/streams/).
  Streams are Redis's log data structure. Understand XADD, XREAD, XREADGROUP,
  XACK, and consumer groups.

- [Enterprise Integration Patterns — Messaging](https://www.enterpriseintegrationpatterns.com/patterns/messaging/).
  The foundational patterns: message channel, message router, publish-subscribe,
  competing consumers, dead letter channel.

- [redis crate documentation](https://docs.rs/redis/latest/redis/).
  Rust Redis client. Focus on the streams API.

- [At-least-once vs exactly-once delivery](https://www.confluent.io/blog/exactly-once-semantics-are-possible-heres-how-apache-kafka-does-it/).
  Why exactly-once is hard and at-least-once with idempotency is the practical choice.

## Key concepts

### Consumer Groups

Multiple consumers can read from the same stream without duplicating work:
- Each consumer in a group gets different messages
- Messages must be acknowledged (XACK) after processing
- Unacknowledged messages can be claimed by other consumers after a timeout
- This provides at-least-once delivery

### Dead Letter Queue

Messages that fail processing after N retries go to a dead letter queue for
manual investigation. This prevents poison messages from blocking the queue.

### Backoff Strategies

```
Constant:     wait 5s, 5s, 5s, ...
Linear:       wait 1s, 2s, 3s, 4s, ...
Exponential:  wait 1s, 2s, 4s, 8s, 16s, ... (with jitter)
```

Always use exponential backoff with jitter for retries.

## Exercises

**Exercise 1**: Use `tokio::sync::broadcast` to implement an in-memory pub/sub.
One producer, three consumers, each receiving all messages.

**Exercise 2**: Use `tokio::sync::mpsc` to implement competing consumers. One
producer, three consumers, each message processed by exactly one consumer.

**Exercise 3**: Implement retry with exponential backoff for a fallible operation.

## You're ready when...

- [ ] You understand pub/sub vs competing consumers
- [ ] You know how Redis Streams consumer groups work
- [ ] You can implement retry with backoff
- [ ] You understand at-least-once delivery semantics

Next: [Project: Message Queue Consumer](../projects/advanced-1/README.md)
