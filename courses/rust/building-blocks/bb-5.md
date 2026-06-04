# PNA Rust &mdash; Building Blocks 5

## Asynchronous Programming in Rust

Put your other projects and concerns aside. Take a breath and relax. Here
are some fun resources for you to explore.

Read all the readings and perform all the exercises.

### Readings

**Async Rust fundamentals**:

- [The Rust Async Book](https://rust-lang.github.io/async-book/).
  Read chapters 1-6. This is the official guide to async programming in Rust.
  Focus on understanding `async`/`await` syntax, `Future` trait, and how
  executors drive futures to completion.

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial).
  Work through the entire tutorial. Tokio is the dominant async runtime in the
  Rust ecosystem and the one used in this project. Pay close attention to:
  spawning tasks, channels, shared state, I/O, and framing.

- [Asynchronous Programming in Rust (mini-book)](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html).
  Covers the mechanics of how `async`/`await` desugars to state machines
  and how the `Future` trait works under the hood.

**Deeper understanding**:

- [Pin and the self-referential struct problem](https://fasterthanli.me/articles/pin-and-suffering).
  A thorough exploration of why `Pin` exists. You don't need to master `Pin` to
  use async Rust, but understanding the motivation helps you debug confusing errors.

- [Tokio: Bridging with sync code](https://tokio.rs/tokio/topics/bridging).
  How to call blocking code from async contexts using `spawn_blocking`. This is
  directly relevant to this project, where file I/O is synchronous but the
  network layer is async.

- [Tower Service trait](https://docs.rs/tower/latest/tower/trait.Service.html).
  Tower is the middleware framework used by tokio-based servers. Understanding
  `Service` will help you in Phase 2 of the course (web development).

**Comparing runtimes**:

- [Tokio vs async-std vs smol](https://kerkour.com/rust-async-await-what-is-a-runtime).
  A comparison of the major async runtimes. We use tokio because it has the
  largest ecosystem and is used by axum, tonic, and most production Rust services.

### Key concepts

**The Future trait**:
```rust
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

You almost never implement `Future` by hand. Instead, use `async fn` and `async` blocks,
which the compiler transforms into `Future`-implementing state machines.

**async/await**:
```rust
// An async function returns impl Future<Output = String>
async fn fetch_data(url: &str) -> String {
    let response = reqwest::get(url).await.unwrap();
    response.text().await.unwrap()
}

// .await suspends the current task until the future completes,
// allowing other tasks to run on the same thread.
```

**Spawning tasks**:
```rust
// spawn() creates a new concurrent task (like a lightweight thread)
let handle = tokio::spawn(async {
    do_something().await
});
let result = handle.await.unwrap();
```

**Blocking in async context**:
```rust
// NEVER do CPU-heavy or blocking I/O directly in an async task.
// Use spawn_blocking to run it on a dedicated thread pool.
let result = tokio::task::spawn_blocking(|| {
    std::fs::read_to_string("large_file.txt")
}).await.unwrap();
```

**Select and Join**:
```rust
// Run multiple futures concurrently, take the first to complete
tokio::select! {
    val = future_a => println!("a completed: {}", val),
    val = future_b => println!("b completed: {}", val),
}

// Run multiple futures concurrently, wait for all
let (a, b) = tokio::join!(future_a, future_b);
```

### Exercises

**Exercise 1**: Write a simple async echo server using `tokio::net::TcpListener`.
Accept connections, read lines, and echo them back. Handle multiple clients
concurrently using `tokio::spawn`.

**Exercise 2**: Write an async function that makes three HTTP-like requests
concurrently (simulate with `tokio::time::sleep`) and returns the first one
to complete using `tokio::select!`.

**Exercise 3**: Create a shared counter using `Arc<Mutex<u64>>` and spawn 10
async tasks that each increment it 100 times. Verify the final count is 1000.

**Exercise 4**: Use `tokio::task::spawn_blocking` to read a file from disk
inside an async function. Compare this with reading it directly (which blocks
the async runtime).

**Exercise 5**: Implement a simple message-passing system using `tokio::sync::mpsc`
channels. One producer task sends messages, multiple consumer tasks process them.

### You're ready when...

- [ ] You can write async functions and use `.await`
- [ ] You understand the difference between `tokio::spawn` and `spawn_blocking`
- [ ] You can handle multiple concurrent connections with tokio
- [ ] You understand why blocking in async context is bad
- [ ] You can use channels for async task communication
- [ ] You're not afraid of `Pin` (even if you don't fully understand it)

Next: [Project 5: Asynchronous KV Store](../projects/project-5/README.md)

---

### Career Checkpoint

You've finished Phase 1. Start building your professional network now — don't
wait until you're job searching. Join the [Rust Discord](https://discord.gg/rust-lang),
subscribe to [This Week in Rust](https://this-week-in-rust.org/), and browse
[r/rust](https://www.reddit.com/r/rust/) regularly. Follow Rust projects on GitHub
that interest you. The goal isn't to contribute yet — it's to absorb how the
community works so you're ready when the time comes in Phase 8.
