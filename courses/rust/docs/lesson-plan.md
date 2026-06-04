# Rust Backend Engineering — Lesson Plan

This is the full lesson plan for the Rust Backend Engineering course. It covers
12 phases, 41 projects, and takes you from Rust beginner to employed remote
backend engineer.

**Estimated total time**: 5-8 months full-time, 8-14 months part-time (including job search).

---

## Phase 0: Rust Foundations

**Goal**: Master Rust's core language features and metaprogramming through five focused projects.
**Time estimate**: 2-4 weeks.
**Prerequisites**: Programming experience in any language.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-0a](../building-blocks/bb-0a.md) | Ownership, borrowing, lifetimes |
| 2 | **Project** | [Ownership Arena](../projects/foundations-1/README.md) | String interner, typed arena, compile-fail tests |
| 3 | Building Block | [bb-0b](../building-blocks/bb-0b.md) | Traits, generics, error handling |
| 4 | **Project** | [Type Machinist](../projects/foundations-2/README.md) | Transform pipeline, thiserror, From/Into |
| 5 | Building Block | [bb-0c](../building-blocks/bb-0c.md) | Iterators, closures |
| 6 | **Project** | [Iterator Forge](../projects/foundations-3/README.md) | Lazy CSV query engine, FromIterator |
| 7 | Building Block | [bb-0d](../building-blocks/bb-0d.md) | Smart pointers, macros |
| 8 | **Project** | [Smart Pointer Workshop](../projects/foundations-4/README.md) | Rc/Arc, RefCell/Mutex, derive macro |
| 9 | Building Block | [bb-0e](../building-blocks/bb-0e.md) | Procedural macros, syn, quote |
| 10 | **Project** | [Procedural Macro Workshop](../projects/foundations-5/README.md) | Builder derive, attribute macro, function-like macro |

---

## Phase 1: Systems Programming

**Goal**: Build a networked, concurrent key-value store from scratch.
**Time estimate**: 4-6 weeks.
**Prerequisites**: Phase 0 or equivalent Rust knowledge.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-1](../building-blocks/bb-1.md) | CLI, Cargo, documentation |
| 2 | **Project** | [The Rust Toolbox](../projects/project-1/README.md) | In-memory KV store, clap, clippy/rustfmt |
| 3 | Building Block | [bb-2](../building-blocks/bb-2.md) | Log-structured storage, serde |
| 4 | **Project** | [Log-Structured File I/O](../projects/project-2/README.md) | Persistent KV store, bitcask, compaction |
| 5 | Building Block | [bb-3](../building-blocks/bb-3.md) | Networking, logging, benchmarking |
| 6 | **Project** | [Synchronous Networking](../projects/project-3/README.md) | Client-server, pluggable engines, criterion |
| 7 | Building Block | [bb-4](../building-blocks/bb-4.md) | Threading, concurrency |
| 8 | **Project** | [Concurrency](../projects/project-4/README.md) | Thread pools, locks, channels, crossbeam |
| 9 | Building Block | [bb-5](../building-blocks/bb-5.md) | Async Rust, tokio |
| 10 | **Project** | [Async KV Store](../projects/project-5/README.md) | tokio, async/await, spawn_blocking |
| 11 | Building Block | [bb-async-adv](../building-blocks/bb-async-adv.md) | Custom futures, Pin/Unpin, executors, backpressure |
| 12 | **Project** | [Advanced Async Deep Dive](../projects/project-6/README.md) | Hand-rolled Future, mini executor, async debugging |

---

## Phase 2: Backend Web Development

**Goal**: Build a production-quality web API with database, auth, and multiple protocols.
**Time estimate**: 4-6 weeks.
**Prerequisites**: Phase 1.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-web-1](../building-blocks/bb-web-1.md) | axum, tower, HTTP APIs |
| 2 | **Project** | [REST API with Axum](../projects/web-1/README.md) | CRUD, validation, pagination, middleware |
| 3 | Building Block | [bb-web-2](../building-blocks/bb-web-2.md) | PostgreSQL, sqlx |
| 4 | **Project** | [Database Layer](../projects/web-2/README.md) | Migrations, repository pattern, transactions |
| 5 | Building Block | [bb-web-3](../building-blocks/bb-web-3.md) | JWT, auth, security |
| 6 | **Project** | [Auth & Authorization](../projects/web-3/README.md) | JWT, argon2, RBAC, refresh tokens, API keys |
| 7 | Building Block | [bb-web-4](../building-blocks/bb-web-4.md) | gRPC, GraphQL |
| 8 | **Project** | [gRPC and GraphQL](../projects/web-4/README.md) | tonic, protobuf, async-graphql, dataloaders |
| 9 | Building Block | [bb-schema](../building-blocks/bb-schema.md) | Expand/contract, safe DDL, backfill strategies |
| 10 | **Project** | [Schema Migrations & Zero-Downtime](../projects/web-5/README.md) | Multi-step migrations, backfill, rollback |

---

## Phase 3: Production Engineering

**Goal**: Make your service production-ready with observability, testing, performance, deployment, debugging, and failure resilience.
**Time estimate**: 5-8 weeks.
**Prerequisites**: Phase 2.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-prod-1](../building-blocks/bb-prod-1.md) | tracing, metrics, OpenTelemetry |
| 2 | **Project** | [Observability Stack](../projects/prod-1/README.md) | Structured logging, Prometheus, health checks |
| 3 | Building Block | [bb-prod-2](../building-blocks/bb-prod-2.md) | Property testing, fuzzing |
| 4 | **Project** | [Testing Mastery](../projects/prod-2/README.md) | proptest, insta, mockall, test strategies |
| 5 | Building Block | [bb-prod-3](../building-blocks/bb-prod-3.md) | Profiling, optimization |
| 6 | **Project** | [Performance & Profiling](../projects/prod-3/README.md) | criterion, flamegraphs, moka caching, Cow |
| 7 | Building Block | [bb-prod-4](../building-blocks/bb-prod-4.md) | Docker, config, deployment |
| 8 | **Project** | [Deployment Pipeline](../projects/prod-4/README.md) | Dockerfile, config crate, graceful shutdown, CI |
| 9 | Building Block | [bb-debug](../building-blocks/bb-debug.md) | GDB, flamegraphs, DHAT, strace, tokio-console |
| 10 | **Project** | [Production Debugging](../projects/prod-5/README.md) | Fix 6 bugs using debuggers, profilers, Miri |
| 11 | Building Block | [bb-chaos](../building-blocks/bb-chaos.md) | Failure patterns, stability patterns, chaos engineering |
| 12 | **Project** | [Chaos Engineering & Failure Recovery](../projects/prod-6/README.md) | Circuit breakers, bulkheads, load shedding, degradation |

---

## Phase 4: Advanced Backend Patterns

**Goal**: Learn the patterns that distinguish senior engineers: event-driven systems,
caching, real-time, and background processing.
**Time estimate**: 4-6 weeks.
**Prerequisites**: Phase 3.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-adv-1](../building-blocks/bb-adv-1.md) | Message queues, event-driven |
| 2 | **Project** | [Message Queue Consumer](../projects/advanced-1/README.md) | Redis streams, consumer groups, dead letters |
| 3 | Building Block | [bb-adv-2](../building-blocks/bb-adv-2.md) | Caching, rate limiting, resilience |
| 4 | **Project** | [Caching & Rate Limiting](../projects/advanced-2/README.md) | Multi-tier cache, token bucket, circuit breaker |
| 5 | Building Block | [bb-adv-3](../building-blocks/bb-adv-3.md) | WebSockets, real-time |
| 6 | **Project** | [WebSocket Real-Time](../projects/advanced-3/README.md) | WS lifecycle, rooms, presence, CRDT |
| 7 | Building Block | [bb-adv-4](../building-blocks/bb-adv-4.md) | Background jobs |
| 8 | **Project** | [Background Job Processor](../projects/advanced-4/README.md) | PostgreSQL queue, SKIP LOCKED, workers |

---

## Phase 5: Distributed Systems

**Goal**: Understand and implement fundamental distributed algorithms and service patterns.
**Time estimate**: 4-8 weeks.
**Prerequisites**: Phase 4.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-dist-1](../building-blocks/bb-dist-1.md) | Raft paper, consensus |
| 2 | **Project** | [Raft Consensus](../projects/dist-1/README.md) | Leader election, log replication, persistence |
| 3 | Building Block | [bb-dist-2](../building-blocks/bb-dist-2.md) | Percolator paper, transactions |
| 4 | **Project** | [Percolator Transactions](../projects/dist-2/README.md) | Snapshot isolation, 2PC, TSO |
| 5 | Building Block | [bb-dist-3](../building-blocks/bb-dist-3.md) | Service discovery, sagas |
| 6 | **Project** | [Service Mesh](../projects/dist-3/README.md) | Registry, load balancing, sagas, idempotency |

---

## Phase 6: Senior Engineering Skills

**Goal**: Develop the judgment, communication, security awareness, and low-level mastery of a senior engineer.
**Time estimate**: 4-6 weeks.
**Prerequisites**: Phase 5.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-sr-1](../building-blocks/bb-sr-1.md) | API design, versioning |
| 2 | **Project** | [API Design & Versioning](../projects/senior-1/README.md) | OpenAPI, utoipa, deprecation, SDK gen |
| 3 | Building Block | [bb-sr-2](../building-blocks/bb-sr-2.md) | Security, OWASP |
| 4 | **Project** | [Security Hardening](../projects/senior-2/README.md) | Sanitization, HMAC, audit logging, cargo-audit |
| 5 | Building Block | [bb-sr-3](../building-blocks/bb-sr-3.md) | System design, interviews |
| 6 | **Project** | [System Design & Interview](../projects/senior-3/README.md) | Design docs, URL shortener, code review |
| 7 | Building Block | [bb-unsafe](../building-blocks/bb-unsafe.md) | Rustonomicon, FFI, Miri, soundness |
| 8 | **Project** | [Unsafe Rust & FFI](../projects/senior-4/README.md) | FFI bindings, allocators, Pin, lock-free, unsafe audit |

---

## Phase 7: Capstone

**Goal**: Integrate everything into a single production-grade system.
**Time estimate**: 2-4 weeks.
**Prerequisites**: All previous phases.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | **Project** | [Production Backend Platform](../projects/capstone/README.md) | Everything |

The capstone has no building block. By this point, you should be self-sufficient.

---

## Phase 8: Open Source & Community

**Goal**: Establish yourself in the Rust open source ecosystem with real contributions and a published crate.
**Time estimate**: 3-4 weeks.
**Prerequisites**: Phase 7.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-oss-1](../building-blocks/bb-oss-1.md) | Finding projects, evaluating repos, contribution types |
| 2 | **Project** | [First Open Source Contribution](../projects/oss-1/README.md) | Fork, fix, submit PR, respond to review |
| 3 | Building Block | [bb-oss-2](../building-blocks/bb-oss-2.md) | Publishing crates, API guidelines, CI, maintenance |
| 4 | **Project** | [Publish a Crate](../projects/oss-2/README.md) | Extract library, document, test, publish to crates.io |

---

## Phase 9: Portfolio & Professional Presence

**Goal**: Make your work visible and demonstrate professional engineering communication skills.
**Time estimate**: 2-3 weeks.
**Prerequisites**: Phase 8.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-portfolio-1](../building-blocks/bb-portfolio-1.md) | GitHub profile, README writing, blog posts |
| 2 | **Project** | [Portfolio Showcase](../projects/portfolio-1/README.md) | Profile README, polished READMEs, deploy capstone, blog post |
| 3 | Building Block | [bb-softskills-1](../building-blocks/bb-softskills-1.md) | RFCs, design docs, code review, async communication |
| 4 | **Project** | [Collaborative Development Simulation](../projects/collab-1/README.md) | Team workflow, PRs, code review, sprint, merge conflicts |

---

## Phase 10: Interview Preparation

**Goal**: Be fully prepared for every stage of the Rust backend engineering interview process.
**Time estimate**: 3-4 weeks.
**Prerequisites**: Phase 9.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-career-1](../building-blocks/bb-career-1.md) | Resume writing, LinkedIn, presenting your work |
| 2 | **Project** | [Technical Interview Gauntlet](../projects/interview-1/README.md) | 20 Rust problems, 5 system designs, code review practice |
| 3 | Building Block | [bb-career-2](../building-blocks/bb-career-2.md) | STAR method, behavioral interviews, remote communication |
| 4 | **Project** | [Behavioral & Remote Interview Prep](../projects/interview-2/README.md) | STAR stories, take-home, mock interviews, follow-up |

---

## Phase 11: Job Search Execution

**Goal**: Land a remote Rust backend engineering job.
**Time estimate**: 4-8 weeks (ongoing until hired).
**Prerequisites**: Phase 10.

| Step | Type | Link | Topics |
|------|------|------|--------|
| 1 | Building Block | [bb-career-3](../building-blocks/bb-career-3.md) | Salary negotiation, offer evaluation, equity |
| 2 | Building Block | [bb-career-4](../building-blocks/bb-career-4.md) | Job search strategy, where Rust jobs are, networking |
| 3 | **Project** | [Job Search Sprint](../projects/job-search-1/README.md) | Target list, applications, networking, tracking |

After completing the job search: see [What's Next](../docs/what-next.md) for
advanced topics, specialization paths, and continuous learning.

---

## Skills Mapped to Job Requirements

After completing this course, you can confidently claim these skills on a resume:

| Job Requirement | Where You Learned It |
|----------------|---------------------|
| Rust proficiency | Phases 0-1 |
| REST API development | Phase 2 (web-1, web-3) |
| PostgreSQL / SQL | Phase 2 (web-2), Phase 4 (advanced-4) |
| gRPC / Protocol Buffers | Phase 2 (web-4) |
| GraphQL | Phase 2 (web-4) |
| Authentication (JWT, OAuth) | Phase 2 (web-3) |
| Message queues (Redis, NATS) | Phase 4 (advanced-1) |
| Caching (Redis, in-memory) | Phase 4 (advanced-2) |
| WebSockets / real-time | Phase 4 (advanced-3) |
| Background job processing | Phase 4 (advanced-4) |
| Observability (logs, metrics, traces) | Phase 3 (prod-1) |
| Testing (unit, integration, property) | Phase 3 (prod-2) |
| Performance optimization | Phase 3 (prod-3) |
| Docker / containerization | Phase 3 (prod-4) |
| CI/CD | Phase 3 (prod-4) |
| Distributed systems (Raft, 2PC) | Phase 5 |
| API design and versioning | Phase 6 (senior-1) |
| Security (OWASP) | Phase 6 (senior-2) |
| System design interviews | Phase 6 (senior-3) |
| Procedural macros (syn, quote) | Phase 0 (foundations-5) |
| Advanced async (futures, Pin, executors) | Phase 1 (project-6) |
| Schema migrations (zero-downtime) | Phase 2 (web-5) |
| Production debugging (GDB, flamegraph, strace) | Phase 3 (prod-5) |
| Chaos engineering (circuit breakers, bulkheads) | Phase 3 (prod-6) |
| Unsafe Rust and FFI | Phase 6 (senior-4) |
| Custom allocators | Phase 6 (senior-4) |
| Lock-free data structures | Phase 6 (senior-4) |
| Open source contributions | Phase 8 (oss-1) |
| Publishing Rust libraries | Phase 8 (oss-2) |
| Technical writing / blog posts | Phase 9 (portfolio-1) |
| RFCs and design docs | Phase 9 (collab-1) |
| Code review (giving and receiving) | Phase 9 (collab-1) |
| Behavioral interview skills | Phase 10 (interview-2) |
| Salary negotiation | Phase 11 (bb-career-3) |
| Job search and networking | Phase 11 (job-search-1) |

---

## How to use this course

1. **Start at your level**: New to Rust → Phase 0. Know Rust basics → Phase 1. Experienced Rustacean → Phase 2.
2. **Read the building block first**: Don't skip the readings. They provide context.
3. **Make the tests pass**: Each project has a test suite. Implement code until all tests pass.
4. **Don't look at solutions**: The learning happens in the struggle.
5. **Move forward even if imperfect**: A passing solution beats a perfect one you never finish.
6. **Build the capstone**: This is where everything comes together. Don't skip it.
