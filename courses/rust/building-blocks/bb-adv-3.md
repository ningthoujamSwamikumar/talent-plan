# Building Block Adv-3: WebSockets and Real-Time Systems

**Prerequisites**: [Project: Caching & Rate Limiting](../projects/advanced-2/README.md).

Before starting [Project: WebSocket Real-Time Service](../projects/advanced-3/README.md),
complete the readings and exercises below.

## What to read

- [WebSocket Protocol (RFC 6455) Summary](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API).
  Understand the upgrade handshake, message frames, ping/pong, and close handshake.

- [Axum WebSocket support](https://docs.rs/axum/latest/axum/extract/ws/index.html).
  Built-in WebSocket extractor for upgrade handling.

- [tokio-tungstenite documentation](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/).
  Low-level async WebSocket client/server.

- [CRDTs for Application Developers](https://www.inkandswitch.com/peritext/).
  Conflict-free replicated data types for concurrent editing without coordination.

- [DashMap documentation](https://docs.rs/dashmap/latest/dashmap/).
  Concurrent HashMap for managing connection state across tasks.

## Key concepts

### WebSocket Lifecycle

```
HTTP Upgrade Request → WebSocket Handshake → Bidirectional Messages → Close
```

### Connection Management

```rust
// Track active connections per room
type Connections = Arc<DashMap<Uuid, broadcast::Sender<Message>>>;

// On connect: add to room's connection set
// On message: broadcast to all connections in room
// On disconnect: remove from connection set
```

### Heartbeat Pattern

```
Server sends Ping every 30s
Client responds with Pong
If no Pong received within 10s → close connection
```

This detects dead connections (network drops, client crashes).

### Backpressure

When a client reads slowly, messages queue up. Without backpressure handling:
- Memory grows unbounded
- Server becomes unresponsive

Solution: bounded channels + drop oldest messages for slow consumers.

## Exercises

**Exercise 1**: Build a chat server with axum WebSockets. Multiple clients
connect, each message is broadcast to all others. Use `tokio::sync::broadcast`.

**Exercise 2**: Add rooms to the chat server. Clients join a room and only
receive messages from that room. Use `DashMap<RoomId, broadcast::Sender>`.

**Exercise 3**: Add ping/pong heartbeat. Disconnect clients that don't respond
within 10 seconds.

## You're ready when...

- [ ] You understand the WebSocket protocol lifecycle
- [ ] You can broadcast messages to connected clients
- [ ] You can manage connection state with DashMap
- [ ] You understand backpressure and heartbeat patterns

Next: [Project: WebSocket Real-Time Service](../projects/advanced-3/README.md)
