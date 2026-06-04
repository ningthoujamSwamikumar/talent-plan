# Project: WebSocket Real-Time Service

**Phase 4, Project 3**

Build a real-time collaboration backend using WebSockets. Students will implement
a multi-room chat and collaborative editing service with presence tracking,
heartbeat monitoring, backpressure handling, and a simple CRDT for concurrent
edits.

## Learning objectives

- Upgrade HTTP connections to WebSocket using axum's built-in extractor
- Manage concurrent connections safely with lock-free data structures
- Design a JSON-based message protocol for bidirectional communication
- Implement presence tracking and heartbeat/keep-alive logic
- Handle backpressure for slow consumers without blocking fast ones
- Apply a basic Conflict-free Replicated Data Type (LWW-Register) for
  concurrent editing

---

## Part 1: WebSocket Upgrade

Handle the HTTP-to-WebSocket upgrade using axum's `WebSocketUpgrade` extractor.

- Create a route `GET /ws` that upgrades the connection.
- Assign each connection a unique `user_id` (UUID v4).
- Split the WebSocket into a sender and receiver half.
- Spawn a task for each half so sending and receiving proceed independently.

**Goal:** a client can connect via WebSocket and the server logs the event.

---

## Part 2: Connection Manager

Track active connections using a `DashMap<RoomId, Vec<Connection>>`.

- Define `Connection` with a `user_id`, a bounded `mpsc::Sender` for outgoing
  messages, and metadata (connected-at timestamp).
- `ConnectionManager` wraps a `DashMap` and provides thread-safe methods to
  add, remove, and look up connections by room.
- Connections are removed on disconnect (explicit leave or detected drop).

**Goal:** the manager can register and unregister connections concurrently.

---

## Part 3: Room System

Implement join/leave semantics and room-scoped broadcasting.

- `join_room(room_id, connection)` adds the connection and notifies existing
  members.
- `leave_room(room_id, user_id)` removes the connection and notifies remaining
  members.
- `broadcast(room_id, message)` sends a `ServerMessage` to every connection in
  the room, skipping the sender when appropriate.

**Goal:** messages are only delivered to members of the target room.

---

## Part 4: Message Protocol

Define the JSON message types that flow over the WebSocket.

### Client -> Server (`ClientMessage`)

| Type      | Fields                                         |
|-----------|-------------------------------------------------|
| `Join`    | `room_id`                                       |
| `Leave`   | `room_id`                                       |
| `Message` | `room_id`, `content`                            |
| `Pong`    |                                                 |
| `Edit`    | `room_id`, `task_id`, `field`, `value`          |

### Server -> Client (`ServerMessage`)

| Type        | Fields                                        |
|-------------|-----------------------------------------------|
| `Joined`    | `room_id`, `members`                          |
| `Left`      | `room_id`, `user_id`                          |
| `Message`   | `room_id`, `user_id`, `content`, `timestamp`  |
| `Presence`  | `room_id`, `members`                          |
| `Ping`      |                                               |
| `Error`     | `message`                                     |
| `EditAck`   | `task_id`, `field`, `value`, `timestamp`      |

All messages are serialized with `serde_json` using the `tag = "type"` attribute
for clean JSON like `{"type":"Join","room_id":"lobby"}`.

---

## Part 5: Presence Tracking

Track which users are in each room and broadcast updates.

- On join, send a `Presence` message to all room members listing the current
  member set.
- On leave (or disconnect), send an updated `Presence` message.
- The `Joined` response includes the current member list so the joining client
  can render it immediately.

**Goal:** every client always knows who else is in the room.

---

## Part 6: Heartbeat

Keep connections alive and detect dead clients.

- The server sends a `Ping` message every 30 seconds to each connected client.
- If the client does not respond with a `Pong` within 10 seconds, the server
  closes the connection and cleans up room memberships.
- Clients should respond to every `Ping` with a `Pong`.

**Goal:** dead connections are detected and removed within ~40 seconds.

---

## Part 7: Backpressure

Protect the server from slow consumers.

- Each connection's outgoing channel is bounded (e.g., capacity 64).
- When the channel is full, the oldest message is dropped (not the newest) so
  the consumer sees the most recent state when it catches up.
- Log a warning when messages are dropped.

**Goal:** a slow client does not block broadcasting to other clients.

---

## Part 8: Simple CRDT -- Last-Writer-Wins Register

Implement a `LwwRegister<T>` for concurrent task description editing.

- Each register holds a `value`, a `timestamp` (milliseconds), and a
  `writer_id`.
- `update` replaces the value only if the new timestamp is strictly greater, or
  if timestamps are equal and the new `writer_id` is lexicographically greater
  (deterministic tie-breaking).
- `merge` combines two registers using the same rule.
- When an `Edit` message arrives, the server updates the register and broadcasts
  an `EditAck` to all room members so every client converges to the same value.

**Goal:** two clients editing the same field concurrently converge to the same
result without coordination.

---

## Testing

Run the test suite:

```bash
cargo test
```

The tests in `tests/websocket_tests.rs` start a real server on a random port and
connect with `tokio-tungstenite` clients. They cover:

1. Basic connectivity and welcome
2. Room join/leave and presence
3. Message broadcasting and room isolation
4. Heartbeat ping/pong and timeout detection
5. Backpressure under slow consumers
6. LWW-Register convergence under concurrent edits
7. Reconnection, error handling, and multi-room scenarios
