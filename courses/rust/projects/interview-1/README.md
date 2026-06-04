# Project: Technical Interview Gauntlet

**Phase 10 — Interview Preparation**

This project puts you through a realistic Rust backend engineering interview
loop. It covers the three types of technical assessment you'll face: coding
problems, system design, and code review. Treat each section as a timed
session — use a timer and don't go over.

## Deliverables

By the end of this project you will have:

- [ ] Solved 20 Rust-specific coding problems (timed)
- [ ] Completed 5 system design exercises with written design docs
- [ ] Reviewed 3 intentionally buggy code samples
- [ ] Done at least 1 mock interview (with a partner or recorded solo)

---

## Part 1: Rust Coding Problems (4 per day for 5 days)

**Rules:**
- Set a timer for 30 minutes per problem.
- Write the solution in Rust. No pseudocode.
- Include tests that verify your solution.
- If you can't solve it in 30 minutes, stop and write what you tried and where
  you got stuck. Then read the solution.

### Ownership and Borrowing

**Problem 1: Flatten References**
Write a function `fn flatten<'a, T>(nested: &'a [&'a [T]]) -> Vec<&'a T>` that
flattens a slice of slices into a single vector of references. The references
must borrow from the original data, not copies.

**Problem 2: Split Mutable**
Implement a function that takes a mutable slice and a predicate, splits the
slice in-place into elements that match and elements that don't (like
`partition` but without allocation), and returns mutable references to both
halves. Hint: think about `split_at_mut`.

**Problem 3: Self-Referential Cache**
Design a struct that caches the result of an expensive computation. The cache
should store a reference to the input AND the output. Why is this hard in Rust?
Write the code that doesn't compile, explain why, then write a version that
works using `std::cell::OnceCell` or owned data.

**Problem 4: Lifetime Debugging**
Fix this code (it won't compile). Explain each error:
```rust
fn longest_word(sentences: &[String]) -> &str {
    let mut longest = "";
    for sentence in sentences {
        for word in sentence.split_whitespace() {
            let owned_word = word.to_string();
            if owned_word.len() > longest.len() {
                longest = &owned_word;
            }
        }
    }
    longest
}
```

### Concurrency and Async

**Problem 5: Thread-Safe Counter**
Implement a `Counter` struct that can be shared across threads. It should support
`increment()`, `decrement()`, and `get()`. Write a test that spawns 100 threads,
each incrementing 1000 times, and verifies the final count is 100,000.

**Problem 6: Async Timeout**
Write an async function `with_timeout<F, T>(future: F, duration: Duration) -> Result<T, TimeoutError>`
that races a future against a timeout. Do NOT use `tokio::time::timeout` — build
it from `tokio::time::sleep` and `tokio::select!`.

**Problem 7: Channel Fan-Out**
Implement a fan-out pattern: one producer sends messages to N consumers via
channels. Each message goes to exactly one consumer (load balancing, not
broadcast). Use `tokio::sync::mpsc`. Include a graceful shutdown mechanism.

**Problem 8: Deadlock Detection**
Here's code that deadlocks. Find and fix the deadlock:
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn transfer(from: &Mutex<i64>, to: &Mutex<i64>, amount: i64) {
    let mut from_balance = from.lock().unwrap();
    let mut to_balance = to.lock().unwrap();
    *from_balance -= amount;
    *to_balance += amount;
}

fn main() {
    let account_a = Arc::new(Mutex::new(1000i64));
    let account_b = Arc::new(Mutex::new(1000i64));
    let a1 = account_a.clone();
    let b1 = account_b.clone();
    let a2 = account_a.clone();
    let b2 = account_b.clone();
    let t1 = thread::spawn(move || transfer(&a1, &b1, 100));
    let t2 = thread::spawn(move || transfer(&b2, &a2, 50));
    t1.join().unwrap();
    t2.join().unwrap();
}
```

### Traits and Generics

**Problem 9: Plugin System**
Design a trait `Plugin` with methods `name() -> &str`, `version() -> (u32, u32, u32)`,
and `execute(&self, input: &str) -> Result<String, PluginError>`. Implement a
`PluginRegistry` that stores `Box<dyn Plugin>`, can register plugins, list them,
and execute a plugin by name. Handle the case where a plugin panics during
execution.

**Problem 10: Type-State Builder**
Implement a builder for an `HttpRequest` that uses the type system to enforce:
you must set the URL before you can set headers, and you must set headers
before you can call `send()`. Calling `send()` without setting the URL should
be a compile-time error, not a runtime error.

**Problem 11: Iterator Adapter**
Implement a custom iterator adapter `fn chunks_by<I, F>(iter: I, predicate: F)`
that groups consecutive elements where the predicate returns true for adjacent
pairs. E.g., `chunks_by([1,2,3,5,6,8], |a, b| b - a == 1)` yields
`[[1,2,3], [5,6], [8]]`.

**Problem 12: Error Type Design**
Design an error type hierarchy for an HTTP API that has: validation errors
(with field-level details), authentication errors, authorization errors, not
found errors, and internal server errors. Each variant should carry context.
Implement `Display`, `Error`, and `From` conversions. Show how a handler
function returns these errors and how they map to HTTP status codes.

### Data Structures and Algorithms (Rust-Flavored)

**Problem 13: LRU Cache**
Implement an LRU cache with O(1) get and put. Use `HashMap` and a doubly-linked
list. The tricky part: how do you handle the borrow checker with a linked list
that's also referenced by a hash map? (Hint: use indices or `slotmap`.)

**Problem 14: Trie with Lifetimes**
Implement a trie that stores `&str` keys (borrowed from the caller) and
arbitrary owned values. The trie should support insert, lookup, prefix search,
and delete. Think carefully about lifetime annotations.

**Problem 15: Concurrent HashMap**
Implement a simple concurrent hash map using sharding (N internal `Mutex<HashMap>`
partitions). Support `get`, `insert`, `remove`, and `len`. Write benchmarks
comparing your implementation against `dashmap` and single-`Mutex` HashMap.

**Problem 16: Stream Processing**
Implement a streaming aggregation: given an async stream of events
`{user_id, event_type, timestamp}`, compute a rolling 5-minute count per user
per event type. Emit an update each time a count changes. Use
`tokio_stream::StreamExt`.

### Unsafe and FFI

**Problem 17: Safe Wrapper**
Write a safe Rust wrapper around this unsafe interface:
```rust
extern "C" {
    fn allocate_buffer(size: usize) -> *mut u8;
    fn free_buffer(ptr: *mut u8, size: usize);
    fn process_buffer(ptr: *const u8, len: usize) -> i32;
}
```
Your wrapper should prevent use-after-free, double-free, and buffer overflow.
Implement `Drop`. Explain each safety invariant in comments.

**Problem 18: Pin Projection**
Explain why this code is unsound and fix it:
```rust
struct SelfRef {
    data: String,
    ptr: *const String,
}
impl SelfRef {
    fn new(data: String) -> Self {
        let mut s = SelfRef { data, ptr: std::ptr::null() };
        s.ptr = &s.data;
        s
    }
    fn get_ref(&self) -> &String {
        unsafe { &*self.ptr }
    }
}
```

### Web / Backend Specific

**Problem 19: Middleware Chain**
Implement a simplified version of tower's `Service` trait and `Layer` pattern.
Create a `Service<Request, Response>` trait, a `LoggingLayer`, a `TimeoutLayer`,
and show how to compose them. Don't use tower itself — build from scratch.

**Problem 20: Database Connection Pool**
Implement a simple async connection pool: a fixed number of connections, async
`acquire()` that waits if all connections are in use, and automatic return on
drop. Use `tokio::sync::Semaphore` and `tokio::sync::Mutex`. Include a health
check that periodically validates connections.

---

## Part 2: System Design Exercises

**Rules:**
- Set a timer for 45 minutes per problem.
- Write a design doc (not code) using the template from bb-softskills-1.
- Include: requirements, capacity estimation, API design, data model,
  architecture diagram (ASCII is fine), and tradeoffs.

**Problem 1: Design a URL Shortener**
- 100M URLs created per month
- 10:1 read-to-write ratio
- URLs expire after 5 years by default
- Custom short URLs supported
- Analytics: click count, geographic distribution, referrer

**Problem 2: Design a Rate Limiting Service**
- Shared across multiple API backends
- Support multiple algorithms (fixed window, sliding window, token bucket)
- 100K rate-limit checks per second
- Must handle distributed deployment (multiple instances)
- Dashboard for monitoring limit usage

**Problem 3: Design a Real-Time Notification System**
- 10M connected users
- Push notifications via WebSocket
- Guaranteed delivery (at-least-once)
- User preferences (which notifications to receive)
- Support mobile push (APNs/FCM) and email fallback

**Problem 4: Design a Distributed Task Queue**
- 1M tasks per day
- Tasks can take 1 second to 1 hour
- Priority levels (critical, high, normal, low)
- Retry with exponential backoff
- Dead letter queue for permanently failed tasks
- Worker auto-scaling

**Problem 5: Design a Multi-Tenant SaaS Backend**
- 1000 tenants, each with 1-10K users
- Data isolation between tenants
- Per-tenant rate limiting and usage billing
- Tenant-specific configuration and feature flags
- Admin API for tenant management

---

## Part 3: Code Review Practice

Review each code sample. Find all bugs, performance issues, security problems,
and style violations. Write your review as GitHub-style inline comments.

**Sample 1: Authentication handler** (find at least 5 issues)
```rust
async fn login(
    State(pool): State<PgPool>,
    Json(body): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        body.email
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    if body.password == user.password_hash {
        let token = jsonwebtoken::encode(
            &Header::default(),
            &Claims { sub: user.id.to_string(), exp: 9999999999 },
            &EncodingKey::from_secret("secret".as_ref()),
        ).unwrap();
        Json(json!({"token": token}))
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
```

**Sample 2: Concurrent data processing** (find at least 4 issues)
```rust
async fn process_batch(items: Vec<Item>) -> Vec<Result<Output, Error>> {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for item in items {
        let results = results.clone();
        handles.push(tokio::spawn(async move {
            let output = expensive_computation(&item).await;
            results.lock().await.push(output);
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Arc::try_unwrap(results).unwrap().into_inner()
}
```

**Sample 3: Cache implementation** (find at least 4 issues)
```rust
struct Cache<V> {
    store: HashMap<String, (V, Instant)>,
    ttl: Duration,
}

impl<V: Clone> Cache<V> {
    fn get(&mut self, key: &str) -> Option<V> {
        if let Some((value, inserted_at)) = self.store.get(key) {
            if inserted_at.elapsed() < self.ttl {
                return Some(value.clone());
            }
        }
        None
    }

    fn set(&mut self, key: String, value: V) {
        self.store.insert(key, (value, Instant::now()));
    }

    fn cleanup(&mut self) {
        let now = Instant::now();
        self.store.retain(|_, (_, t)| now.duration_since(*t) < self.ttl);
    }
}
```

---

## Part 4: Mock Interview

Do at least one full mock interview (60-90 minutes):

**If you have a partner:**
Take turns interviewing each other. One round of coding (30 min), one round of
system design (30 min), one round of behavioral (15 min).

**If solo:**
Record yourself (screen + audio). Set up a problem you haven't solved yet.
Talk out loud as you solve it — this is what interviewers are listening for.
Review the recording afterward and note where you got stuck or went silent.

**What interviewers evaluate:**

1. **Communication** — Do you think out loud? Do you ask clarifying questions?
2. **Problem decomposition** — Do you break the problem into manageable pieces?
3. **Trade-off awareness** — Do you mention alternatives and why you chose this
   approach?
4. **Code quality** — Is your code readable? Do you handle errors?
5. **Testing mindset** — Do you mention edge cases? Do you verify your solution?

## Success Criteria

- All 20 coding problems attempted (solved or documented where stuck)
- 5 system design docs written with architecture diagrams
- 3 code reviews completed with specific, actionable feedback
- At least 1 mock interview completed
