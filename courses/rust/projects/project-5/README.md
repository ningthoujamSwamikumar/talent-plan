# Project 5: Asynchronous Key-Value Store

**Task**: Convert the multi-threaded, persistent key/value store server and client
to use _asynchronous_ networking with tokio and `async`/`await`.

**Goals**:

- Write and compose async functions with `async`/`await`
- Perform asynchronous networking with the tokio runtime
- Bridge sync file I/O with async networking using `spawn_blocking`
- Understand task spawning and concurrent request handling
- Handle errors in async contexts

**Topics**: asynchrony, futures, tokio, `async`/`await`, `spawn_blocking`.

- [Introduction](#user-content-introduction)
- [Project spec](#user-content-project-spec)
- [Project setup](#user-content-project-setup)
- [Part 1: Async client with tokio](#user-content-part-1-async-client-with-tokio)
- [Part 2: Async server with tokio](#user-content-part-2-async-server-with-tokio)
- [Part 3: Bridging sync engines with async](#user-content-part-3-bridging-sync-engines-with-async)
- [Part 4: Shared thread pool with async engine](#user-content-part-4-shared-thread-pool-with-async-engine)
- [Part 5: Benchmarking async vs sync](#user-content-part-5-benchmarking-async-vs-sync)


## Introduction

In the previous project you built a multi-threaded server using a thread pool.
Each client connection was handled by a dedicated thread. This works well for
moderate load, but threads are expensive: each one uses significant memory for its
stack, and the OS scheduler has limits on how many it can manage efficiently.

Asynchronous I/O solves this by letting a small number of threads handle thousands
of concurrent connections. Instead of blocking a thread while waiting for network
data, an async runtime (tokio) suspends the current task and runs another one. When
data arrives, the original task resumes where it left off.

In this project you will convert your key-value store to async networking while
keeping the storage engine synchronous. This is a realistic architecture — network
I/O benefits enormously from async, but file I/O on most operating systems is still
best handled with blocking calls on a thread pool.

Be sure to complete [Building Block 5](../../building-blocks/bb-5.md) before starting
this project. Async Rust has a steep learning curve, but the patterns become natural
with practice.


## Project spec

The cargo project, `kvs`, builds a command-line key-value store client called
`kvs-client`, and a key-value store server called `kvs-server`, both of which in
turn call into a library called `kvs`. The client speaks to the server over
a custom protocol.

The CLI interface is identical to [Project 4][previous project]. The storage engines
(`KvStore` and `SledKvsEngine`) remain synchronous internally.

The changes in this project:

1. `KvsClient` uses `tokio::net::TcpStream` for async networking
2. `KvsServer` uses `tokio::net::TcpListener` and spawns async tasks per connection
3. `KvsEngine` gains an async wrapper that bridges to the sync implementation via
   `tokio::task::spawn_blocking`
4. The `ThreadPool` becomes `Clone + Send + Sync + 'static` so it can be shared
   across async tasks

[previous project]: ../project-4/README.md


## Project setup

Continuing from your previous project, delete your previous `tests` directory and
copy this project's `tests` directory into its place.

Update your `Cargo.toml` dependencies:

```toml
[dependencies]
clap = "4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sled = "0.34"
log = "0.4"
env_logger = "0.10"
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
assert_cmd = "2.0"
criterion = { version = "0.5", features = ["async_tokio"] }
predicates = "3.0"
rand = "0.8"
tempfile = "3.0"
walkdir = "2.2"
```

Note: We use `tokio` 1.x with `async`/`await` syntax throughout. If you see older
resources using `futures` 0.1 or hand-written `Future` implementations, ignore them —
`async`/`await` is the modern approach.


## Part 1: Async client with tokio

Convert `KvsClient` to use async networking.

**Step 1**: Change the `main` function of `kvs-client` to use `#[tokio::main]`:

```rust
#[tokio::main]
async fn main() {
    // your existing CLI parsing...
    // but now you can call async functions
}
```

**Step 2**: Convert `KvsClient::connect` to an async function:

```rust
impl KvsClient {
    pub async fn connect(addr: SocketAddr) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        // ...
    }
}
```

Replace `std::net::TcpStream` with `tokio::net::TcpStream`. Replace synchronous
`read`/`write` calls with their async equivalents (`.await`).

**Step 3**: Convert `get`, `set`, and `remove` to async methods:

```rust
pub async fn get(&mut self, key: String) -> Result<Option<String>> {
    // send request, await response
}
```

The protocol (serialization format over TCP) stays the same — you're only changing
how bytes are sent and received.

**Step 4**: Use `tokio::io::AsyncReadExt` and `tokio::io::AsyncWriteExt` for
reading and writing. If you were using `serde_json::from_reader` with a synchronous
reader, you'll need to read bytes into a buffer first and then deserialize:

```rust
use tokio::io::{AsyncReadExt, AsyncWriteExt};

// Reading: buffer the data, then deserialize
let mut buf = vec![0u8; 1024];
let n = stream.read(&mut buf).await?;
let response: Response = serde_json::from_slice(&buf[..n])?;
```

Consider using a length-prefix framing protocol: write the message length as a 4-byte
big-endian u32, then the message bytes. This avoids the "how much to read" problem.

At this point, run the CLI tests:

```
cargo test --test cli
```

The client tests should pass. The server is still synchronous.


## Part 2: Async server with tokio

Convert `KvsServer` to use tokio's async networking.

**Step 1**: Change `kvs-server`'s `main` to `#[tokio::main]`:

```rust
#[tokio::main]
async fn main() {
    // ...
}
```

**Step 2**: Replace `std::net::TcpListener` with `tokio::net::TcpListener`:

```rust
let listener = TcpListener::bind(addr).await?;

loop {
    let (stream, _addr) = listener.accept().await?;
    // handle connection...
}
```

**Step 3**: Spawn each connection as a tokio task:

```rust
loop {
    let (stream, _addr) = listener.accept().await?;
    let engine = engine.clone();

    tokio::spawn(async move {
        if let Err(e) = handle_connection(engine, stream).await {
            log::error!("Connection error: {}", e);
        }
    });
}
```

This is the key difference from the thread pool approach: `tokio::spawn` creates a
lightweight task (not a full OS thread), and tokio multiplexes thousands of tasks onto
a small number of threads.

**Step 4**: Convert `handle_connection` (or whatever you called your per-connection
handler) to an async function. Read requests and write responses using async I/O.

**Important**: Your `KvsEngine` methods are still synchronous. For now, call them
directly from the async handler. This will _block_ the tokio runtime thread — we'll
fix this in Part 3.

Run the full test suite:

```
cargo test
```

The server should handle concurrent connections, even though the engine calls block.


## Part 3: Bridging sync engines with async

Your storage engines (`KvStore` and `SledKvsEngine`) perform file I/O, which is
blocking. Calling blocking code directly inside a `tokio::spawn`ed task is bad —
it prevents that thread from handling other connections.

The solution: `tokio::task::spawn_blocking`.

**Step 1**: Create an async wrapper for `KvsEngine`:

```rust
/// An async wrapper around a synchronous KvsEngine.
#[derive(Clone)]
pub struct AsyncKvsEngine<E: KvsEngine> {
    inner: E,
}

impl<E: KvsEngine> AsyncKvsEngine<E> {
    pub fn new(engine: E) -> Self {
        Self { inner: engine }
    }

    pub async fn get(&self, key: String) -> Result<Option<String>> {
        let engine = self.inner.clone();
        tokio::task::spawn_blocking(move || engine.get(key))
            .await
            .unwrap() // unwrap the JoinError
    }

    pub async fn set(&self, key: String, value: String) -> Result<()> {
        let engine = self.inner.clone();
        tokio::task::spawn_blocking(move || engine.set(key, value))
            .await
            .unwrap()
    }

    pub async fn remove(&self, key: String) -> Result<()> {
        let engine = self.inner.clone();
        tokio::task::spawn_blocking(move || engine.remove(key))
            .await
            .unwrap()
    }
}
```

For this to work, your `KvsEngine` implementations must be `Clone + Send + 'static`.
If they aren't already (from Project 4), make them so. Typically this means wrapping
the engine's internal state in `Arc`.

**Step 2**: Use `AsyncKvsEngine` in your server's connection handler instead of
calling engine methods directly.

**Step 3**: Run the tests again:

```
cargo test
```

The behavior should be identical, but now your async runtime threads are never blocked
by file I/O.

### Think about the architecture

You now have two layers of concurrency:

1. **Tokio runtime threads** (typically one per CPU core): Handle async networking.
   These threads are never blocked — they process thousands of connections concurrently.

2. **Blocking thread pool** (managed by tokio's `spawn_blocking`): Handle synchronous
   file I/O. These threads may block waiting for disk operations.

This is a common production pattern. The async layer handles the "C10K problem"
(many concurrent connections with mostly idle I/O), while the blocking layer handles
work that can't be made async (file I/O, CPU-heavy computation, FFI calls).


## Part 4: Shared thread pool with async engine

In Project 4, you implemented multiple `ThreadPool` variants. Now that tokio provides
its own thread management, consider how the pieces fit together.

**Step 1**: Ensure your `ThreadPool` trait and implementations are `Clone + Send + Sync + 'static`:

```rust
pub trait ThreadPool: Clone + Send + 'static {
    fn new(threads: u32) -> Result<Self>;
    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static;
}
```

**Step 2**: As an alternative to `spawn_blocking`, try using your own `ThreadPool`
to run engine operations:

```rust
impl<E: KvsEngine> AsyncKvsEngine<E> {
    pub async fn get_with_pool<P: ThreadPool>(
        &self,
        pool: &P,
        key: String,
    ) -> Result<Option<String>> {
        let (tx, rx) = tokio::sync::oneshot::channel();
        let engine = self.inner.clone();
        pool.spawn(move || {
            let result = engine.get(key);
            let _ = tx.send(result);
        });
        rx.await.unwrap()
    }
}
```

This demonstrates how async and sync thread pools can interoperate using channels.

**Step 3**: Run the benchmarks to compare the approaches:

```
cargo bench
```

Is `spawn_blocking` or your custom thread pool faster? Think about why.

### Why this matters

In production, you'll regularly encounter this pattern: an async service that needs
to call into synchronous libraries (database drivers, file systems, compression,
legacy code). Knowing how to bridge the two worlds correctly — without accidentally
blocking your async runtime — is a critical skill.


## Part 5: Benchmarking async vs sync

Write criterion benchmarks comparing your Project 4 (sync) and Project 5 (async)
implementations.

**Step 1**: Create benchmarks for concurrent reads and writes:

```rust
fn bench_async_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("async_get_100_concurrent", |b| {
        b.iter(|| {
            rt.block_on(async {
                // spawn 100 concurrent get requests
            })
        })
    });
}
```

**Step 2**: Benchmark with varying levels of concurrency (1, 10, 100, 1000 concurrent
requests). The async version should show better performance at high concurrency
because it doesn't create a thread per connection.

**Step 3**: Document your findings. At what concurrency level does async become
faster? What are the tradeoffs?

Run the benchmarks:

```
cargo bench
```

---

Nice coding, friend. You've now built a complete networked key-value store that
progressed from a simple CLI tool to an asynchronous server. The patterns you've
learned — async networking, sync-to-async bridging, concurrent task management —
are the foundation of every production Rust backend service.

Take a well-deserved break, then continue to [Phase 2](../../README.md) where you'll
apply these skills to build real-world web services.


## Extension 1: Graceful shutdown

Implement graceful shutdown for the server:

1. Listen for `SIGTERM` using `tokio::signal::ctrl_c()`
2. Stop accepting new connections
3. Wait for in-flight requests to complete (with a timeout)
4. Flush any pending writes and clean up

This is a pattern you'll use in every production server.


## Extension 2: Connection multiplexing

Instead of one request per connection, allow clients to send multiple requests over
a single TCP connection (pipelining). Use tokio's `Framed` codec with
`tokio_util::codec` to handle message framing cleanly.

This improves throughput by amortizing connection setup cost.
