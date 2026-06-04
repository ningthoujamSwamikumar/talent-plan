# Phase 6, Project 3: System Design and Interview Prep

This project bridges the gap between writing code and thinking about systems at scale.
You will design and implement two classic system design problems (URL shortener and API
gateway), practice code review on a deliberately flawed codebase, and work through
Rust-specific interview questions with model answers.

## Prerequisites

- Completion of all Phase 5 and Phase 6 projects (senior-1, senior-2)
- Strong understanding of axum, async Rust, and data structures
- Familiarity with system design concepts (load balancing, caching, rate limiting)

## Part 1: URL Shortener Design

**Goal:** Write a design document and then implement a URL shortener service.

### Design Document (write this first)

Before coding, write a short design document (as comments at the top of `src/url_shortener.rs`)
covering:

1. **Capacity Estimation:**
   - 100M URLs created per month, 10:1 read-to-write ratio
   - How much storage per URL? Total storage for 5 years?
   - Estimate QPS for reads and writes

2. **API Design:**
   - `POST /shorten` — takes a long URL, returns a short code
   - `GET /:code` — redirects to the original URL
   - `GET /:code/stats` — returns click count and creation time

3. **Data Model:**
   - What fields does each shortened URL need?
   - How do you generate unique short codes?
   - What index strategy for fast lookups?

4. **Architecture:**
   - Single-node (for this implementation) vs. distributed (in your design doc)
   - Where would you add caching?
   - How would you handle hash collisions?

### Implementation (~300 lines)

**Tasks:**
1. Implement `UrlShortener` struct with in-memory storage
2. Short code generation: base62 encoding of a counter or random bytes
3. `POST /shorten` handler — create a new short URL
4. `GET /:code` handler — redirect (HTTP 302) to the original URL
5. `GET /:code/stats` handler — return visit count and metadata
6. Increment visit count atomically on each redirect
7. Handle edge cases: duplicate URLs, invalid codes, empty input

## Part 2: Rate-Limited API Gateway

**Goal:** Design and implement an API gateway that routes requests to backend services
with per-client rate limiting.

### Design Document

Write a design document covering:

1. **Requirements:**
   - Route requests to different backends based on URL prefix
   - Apply per-client rate limits (token bucket algorithm)
   - Forward headers, preserve request body
   - Return 429 Too Many Requests when rate limit is exceeded

2. **Rate Limiting Strategy:**
   - Token bucket: capacity, refill rate, per-client state
   - How to identify clients (API key header or IP)
   - What happens at the limit boundary?

3. **Architecture:**
   - Middleware pipeline: auth -> rate limit -> route -> forward
   - How would this scale horizontally? (sticky sessions, shared state)

### Implementation

**Tasks:**
1. Implement `TokenBucket` struct: `try_acquire() -> bool`, automatic refill over time
2. Implement `RateLimiter` that maps client IDs to token buckets
3. Implement `GatewayConfig` with routing rules (prefix -> backend URL)
4. Implement the gateway router that checks rate limits, then forwards requests
5. Return proper 429 responses with `Retry-After` header
6. Clean up expired client buckets periodically

## Part 3: Code Review Exercise

**Goal:** Review a provided file (`src/review_exercise.rs`) that contains intentional bugs,
style issues, performance problems, and security vulnerabilities. Write your review as
comments in the test file.

The file simulates a "pull request" for a user authentication module. It contains
approximately 15 issues across these categories:

- **Bugs:** Off-by-one errors, incorrect error handling, logic errors
- **Security:** Plaintext password storage, timing attacks, missing input validation
- **Performance:** Unnecessary cloning, O(n) lookups where O(1) is possible
- **Style:** Inconsistent naming, missing documentation, dead code
- **Rust-specific:** Unwrap in production code, unused Results, Send/Sync issues

Your job is to find and document as many issues as you can. The answer key is provided
in comments at the bottom of the test file.

## Part 4: Interview Question Practice

Work through these 10 Rust-specific interview questions. Model answers are provided in
`src/lib.rs` as documentation comments, but try to answer them yourself first.

1. **Ownership & Borrowing:** Explain why this code does not compile and how to fix it.
2. **Lifetimes:** What lifetime annotations are needed for a function that returns a
   reference to the longer of two string slices?
3. **Send & Sync:** What are `Send` and `Sync`? Give an example of a type that is `Send`
   but not `Sync`.
4. **Async Runtime:** What happens if you call a blocking function inside `tokio::spawn`?
   How do you fix it?
5. **Error Handling:** Compare `unwrap()`, `expect()`, `?`, and `match` for error handling.
   When is each appropriate?
6. **Smart Pointers:** Explain the difference between `Box<T>`, `Rc<T>`, and `Arc<T>`.
   When would you use each?
7. **Trait Objects vs Generics:** When would you use `dyn Trait` vs `impl Trait` vs `<T: Trait>`?
   What are the trade-offs?
8. **Interior Mutability:** Explain `Cell<T>`, `RefCell<T>`, and `Mutex<T>`. Why does Rust
   need these?
9. **Pin & Unpin:** What problem does `Pin<T>` solve? When do you encounter it in async Rust?
10. **Unsafe:** Name three things that require `unsafe` in Rust. How do you minimize the
    blast radius of unsafe code?

## Testing

Run all tests:
```
cargo test
```

Tests cover:
- URL shortener: creation, redirect, stats, duplicates, invalid codes (~10 tests)
- API gateway: routing, rate limiting, 429 responses, header forwarding (~10 tests)

## What You Will Learn

- System design methodology: estimation, API design, data modeling, architecture
- Token bucket rate limiting algorithm
- Code review skills and common code smells
- Rust-specific interview topics and best practices
- Translating high-level design into working code
