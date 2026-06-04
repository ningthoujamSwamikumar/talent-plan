# Rust Backend Engineering: From Beginner to Senior

A comprehensive training course that takes you from Rust beginner to production-ready
senior backend engineer through 26 hands-on projects across 8 phases.

Over a series of progressively challenging projects, you will build systems software,
web APIs, production infrastructure, and distributed systems in Rust. Each project
includes a complete test suite — you learn by making tests pass. In between projects
are building blocks: curated readings and exercises on the subjects necessary to
complete the next project.

**[View the full lesson plan][plan]**.


## What this course covers

### Phase 0: Rust Foundations
Ownership, borrowing, lifetimes, traits, generics, iterators, closures, smart pointers,
and macros. Four focused projects for programmers new to Rust.

### Phase 1: Systems Programming
Build a networked, multithreaded, asynchronous key-value store from scratch across five
projects. Covers serialization, file I/O, networking, concurrency, and async Rust.

### Phase 2: Backend Web Development
Build a task management API with axum, PostgreSQL (sqlx), JWT authentication, gRPC
(tonic), and GraphQL (async-graphql). Four projects covering the full backend stack.

### Phase 3: Production Engineering
Make your service production-grade: observability (tracing + metrics), advanced testing
(property-based, fuzzing, snapshots), performance profiling, and deployment (Docker, CI/CD,
graceful shutdown).

### Phase 4: Advanced Backend Patterns
Message queues and event-driven architecture, caching and rate limiting, WebSockets and
real-time systems, background job processing. The patterns that distinguish senior from
mid-level engineers.

### Phase 5: Distributed Systems
Implement the Raft consensus algorithm, Percolator distributed transactions, and build
a service mesh with service discovery, distributed tracing, and saga orchestration.

### Phase 6: Senior Engineering Skills
API design and versioning with OpenAPI, security hardening (OWASP), system design
practice, and interview preparation.

### Phase 7: Capstone
Build a complete, production-grade, real-time collaborative project management platform
integrating every skill from the course. Evaluated on correctness, performance, and
operational readiness.


## The goal of this course

The goal is to prepare you for a **senior remote Rust backend engineer role**. After
completing this course, you will be able to:

- Design and build production REST, gRPC, and GraphQL APIs
- Work with PostgreSQL, Redis, and message queues
- Implement authentication, authorization, and security hardening
- Write comprehensive tests (unit, integration, property-based, fuzzing)
- Profile and optimize Rust services for performance
- Deploy containerized services with observability
- Build distributed systems with consensus and distributed transactions
- Make architectural decisions and articulate tradeoffs
- Pass system design interviews and Rust-specific technical interviews

This is not a theoretical course. Every concept is practiced through building real
software with real test suites.


## Who is this for?

This course is designed for:

- **Programmers new to Rust** who want to become professional Rust backend engineers
- **Backend engineers** from other languages (Go, Python, TypeScript, Java) transitioning to Rust
- **Systems programmers** who want to add web backend skills
- **Students** preparing for Rust backend engineering roles

If you have programming experience in any language and can use a terminal and git,
you can start this course. Phase 0 teaches Rust fundamentals — you don't need prior
Rust experience, though reading [The Rust Book] first is recommended.


## Prerequisites

- [ ] Intermediate programming experience in any language
- [ ] Comfortable with the terminal and command line
- [ ] Know how to use [git]
- [ ] **Recommended**: Read [The Rust Book] (at minimum chapters 1-10)
- [ ] **For Phase 2+**: Basic SQL knowledge
- [ ] **For Phase 5**: Understanding of networking fundamentals

New to Rust? Start with Phase 0. Already know Rust basics? Start with Phase 1.
See [prerequisites][pre] for detailed guidance.


## Course structure

Each phase consists of:

1. **Building blocks**: Readings and small exercises to prepare for the project
2. **Projects**: Scaffolded Cargo projects with complete test suites. You implement
   the code to make the tests pass.

Projects are self-contained. Each includes a `README.md` with a multi-part walkthrough,
`Cargo.toml` with all dependencies, source stubs with `todo!()` markers, and a
comprehensive test suite.

**Estimated time**: 3-6 months full-time, or 6-12 months part-time.


## All projects

| # | Phase | Project | Key Topics |
|---|-------|---------|------------|
| 0a | Foundations | [Ownership Arena](projects/foundations-1/README.md) | Ownership, borrowing, lifetimes |
| 0b | Foundations | [Type Machinist](projects/foundations-2/README.md) | Traits, generics, error handling |
| 0c | Foundations | [Iterator Forge](projects/foundations-3/README.md) | Iterators, closures, lazy evaluation |
| 0d | Foundations | [Smart Pointer Workshop](projects/foundations-4/README.md) | Smart pointers, interior mutability, macros |
| 1 | Systems | [The Rust Toolbox](projects/project-1/README.md) | Cargo, CLI, data structures |
| 2 | Systems | [Log-Structured File I/O](projects/project-2/README.md) | Serialization, file I/O, error handling |
| 3 | Systems | [Synchronous Networking](projects/project-3/README.md) | TCP, traits, benchmarking |
| 4 | Systems | [Concurrency](projects/project-4/README.md) | Thread pools, locks, channels |
| 5 | Systems | [Async KV Store](projects/project-5/README.md) | tokio, async/await, spawn_blocking |
| 6 | Web | [REST API with Axum](projects/web-1/README.md) | axum, tower, validation, CORS |
| 7 | Web | [Database Layer](projects/web-2/README.md) | sqlx, PostgreSQL, migrations, transactions |
| 8 | Web | [Auth & Authorization](projects/web-3/README.md) | JWT, argon2, RBAC |
| 9 | Web | [gRPC and GraphQL](projects/web-4/README.md) | tonic, async-graphql, dataloaders |
| 10 | Production | [Observability Stack](projects/prod-1/README.md) | tracing, Prometheus, health checks |
| 11 | Production | [Testing Mastery](projects/prod-2/README.md) | proptest, fuzzing, insta, mockall |
| 12 | Production | [Performance & Profiling](projects/prod-3/README.md) | criterion, flamegraphs, caching |
| 13 | Production | [Deployment Pipeline](projects/prod-4/README.md) | Docker, config, graceful shutdown, CI/CD |
| 14 | Advanced | [Message Queue Consumer](projects/advanced-1/README.md) | Redis streams, consumer groups, dead letters |
| 15 | Advanced | [Caching & Rate Limiting](projects/advanced-2/README.md) | Multi-tier cache, token bucket, circuit breaker |
| 16 | Advanced | [WebSocket Real-Time](projects/advanced-3/README.md) | WebSockets, DashMap, CRDT |
| 17 | Advanced | [Background Job Processor](projects/advanced-4/README.md) | PostgreSQL job queue, SKIP LOCKED, workers |
| 18 | Distributed | [Raft Consensus](projects/dist-1/README.md) | Leader election, log replication |
| 19 | Distributed | [Percolator Transactions](projects/dist-2/README.md) | Snapshot isolation, 2PC |
| 20 | Distributed | [Service Mesh](projects/dist-3/README.md) | Service discovery, sagas, distributed tracing |
| 21 | Senior | [API Design & Versioning](projects/senior-1/README.md) | OpenAPI, utoipa, backward compatibility |
| 22 | Senior | [Security Hardening](projects/senior-2/README.md) | OWASP, sanitization, HMAC, audit logging |
| 23 | Senior | [System Design & Interview](projects/senior-3/README.md) | Design docs, capacity estimation, code review |
| 24 | Capstone | [Production Backend Platform](projects/capstone/README.md) | Everything combined |


## Other courses in this series

This course is part of a [series of courses] initiated by [PingCAP]. The distributed
systems course at [courses/dss](../dss/README.md) provides the Raft and Percolator
labs referenced in Phase 5.


## Contributing

See [CONTRIBUTING.md].


## License

All text and code for this course is dual licensed [CC-BY 4.0] and [MIT].


<!-- links -->

[CONTRIBUTING.md]: CONTRIBUTING.md
[CC-BY 4.0]: https://opendefinition.org/licenses/cc-by/
[MIT]: https://opensource.org/licenses/MIT
[PingCAP]: https://pingcap.com/
[The Rust Book]: https://doc.rust-lang.org/stable/book/
[git]: https://git-scm.com/
[plan]: ./docs/lesson-plan.md
[pre]: ./docs/prerequisites.md
[series of courses]: https://github.com/pingcap/talent-plan/
