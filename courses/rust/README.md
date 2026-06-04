# Rust Backend Engineering: From Beginner to Hired

A comprehensive training course that takes you from Rust beginner to employed
remote Rust backend engineer through 41 hands-on projects across 12 phases.

Over a series of progressively challenging projects, you will build systems software,
web APIs, production infrastructure, and distributed systems in Rust. Then you'll
build your professional presence, contribute to open source, prepare for interviews,
and execute a real job search. Each technical project includes a complete test suite
— you learn by making tests pass. Career projects have concrete deliverables: merged
PRs, published crates, polished portfolios, and sent applications.

**[View the full lesson plan][plan]**.


## What this course covers

### Phase 0: Rust Foundations
Ownership, borrowing, lifetimes, traits, generics, iterators, closures, smart pointers,
macros, and procedural macros. Five focused projects for programmers new to Rust.

### Phase 1: Systems Programming
Build a networked, multithreaded, asynchronous key-value store from scratch across six
projects. Covers serialization, file I/O, networking, concurrency, async Rust, and
advanced async internals (custom futures, Pin, executors, backpressure).

### Phase 2: Backend Web Development
Build a task management API with axum, PostgreSQL (sqlx), JWT authentication, gRPC
(tonic), and GraphQL (async-graphql). Then master schema migrations and zero-downtime
deploys. Five projects covering the full backend stack.

### Phase 3: Production Engineering
Make your service production-grade: observability (tracing + metrics), advanced testing
(property-based, fuzzing, snapshots), performance profiling, deployment (Docker, CI/CD,
graceful shutdown), production debugging (GDB, flamegraphs, strace, Miri), and chaos
engineering (circuit breakers, bulkheads, failure recovery).

### Phase 4: Advanced Backend Patterns
Message queues and event-driven architecture, caching and rate limiting, WebSockets and
real-time systems, background job processing. The patterns that distinguish senior from
mid-level engineers.

### Phase 5: Distributed Systems
Implement the Raft consensus algorithm, Percolator distributed transactions, and build
a service mesh with service discovery, distributed tracing, and saga orchestration.

### Phase 6: Senior Engineering Skills
API design and versioning with OpenAPI, security hardening (OWASP), system design
practice, interview preparation, unsafe Rust, FFI, custom allocators, and lock-free
data structures.

### Phase 7: Capstone
Build a complete, production-grade, real-time collaborative project management platform
integrating every skill from the course. Evaluated on correctness, performance, and
operational readiness.

### Phase 8: Open Source & Community
Make your first real contribution to a Rust open source project. Then extract a
reusable library from your course work and publish it to crates.io. Deliverables:
a submitted PR and a published crate.

### Phase 9: Portfolio & Professional Presence
Set up your GitHub profile, polish project READMEs, deploy your capstone live, write
a technical blog post, and simulate a team development workflow with RFCs, code
reviews, and sprints.

### Phase 10: Interview Preparation
20 Rust-specific coding problems, 5 system design exercises, code review practice,
behavioral interview prep with STAR stories, a timed take-home assignment, and mock
interviews.

### Phase 11: Job Search Execution
Build a target company list, finalize your resume, send real applications, network in
the Rust community, and track your progress through the hiring pipeline. This phase
runs until you accept an offer.


## The goal of this course

The goal is to get you **hired as a remote Rust backend engineer**. Not just to teach
you Rust — to get you from zero to employed. After completing this course, you will
be able to:

- Design and build production REST, gRPC, and GraphQL APIs
- Work with PostgreSQL, Redis, and message queues
- Implement authentication, authorization, and security hardening
- Write comprehensive tests (unit, integration, property-based, fuzzing)
- Profile and optimize Rust services for performance
- Deploy containerized services with observability
- Build distributed systems with consensus and distributed transactions
- Make architectural decisions and articulate tradeoffs
- Contribute to real open source Rust projects with merged PRs
- Publish and maintain your own crate on crates.io
- Present your work through a professional portfolio and technical writing
- Pass system design, coding, and behavioral interviews
- Run a disciplined job search and negotiate offers

This is not a theoretical course. Every concept is practiced through building real
software, contributing to real projects, and doing real job search activities.


## Who is this for?

This course is designed for:

- **Programmers new to Rust** who want to become professional Rust backend engineers
- **Backend engineers** from other languages (Go, Python, TypeScript, Java) transitioning to Rust
- **Systems programmers** who want to add web backend skills
- **Students** preparing for Rust backend engineering roles

If you have programming experience in any language and can use a terminal and git,
you can start this course. Phase 0 teaches Rust fundamentals — you don't need prior
Rust experience, though reading [The Rust Book] first is recommended.

No prior professional experience is required. This course is specifically designed
to give you everything you need — technical skills, portfolio, open source track
record, and job search skills — to land your first remote Rust engineering role.


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

**Estimated time**: 6-10 months full-time, or 10-16 months part-time (including job search).


## All projects

| # | Phase | Project | Key Topics |
|---|-------|---------|------------|
| 0a | Foundations | [Ownership Arena](projects/foundations-1/README.md) | Ownership, borrowing, lifetimes |
| 0b | Foundations | [Type Machinist](projects/foundations-2/README.md) | Traits, generics, error handling |
| 0c | Foundations | [Iterator Forge](projects/foundations-3/README.md) | Iterators, closures, lazy evaluation |
| 0d | Foundations | [Smart Pointer Workshop](projects/foundations-4/README.md) | Smart pointers, interior mutability, macros |
| 0e | Foundations | [Procedural Macro Workshop](projects/foundations-5/README.md) | Derive macros, attribute macros, syn, quote |
| 1 | Systems | [The Rust Toolbox](projects/project-1/README.md) | Cargo, CLI, data structures |
| 2 | Systems | [Log-Structured File I/O](projects/project-2/README.md) | Serialization, file I/O, error handling |
| 3 | Systems | [Synchronous Networking](projects/project-3/README.md) | TCP, traits, benchmarking |
| 4 | Systems | [Concurrency](projects/project-4/README.md) | Thread pools, locks, channels |
| 5 | Systems | [Async KV Store](projects/project-5/README.md) | tokio, async/await, spawn_blocking |
| 6 | Systems | [Advanced Async Deep Dive](projects/project-6/README.md) | Custom futures, Pin, executors, backpressure |
| 7 | Web | [REST API with Axum](projects/web-1/README.md) | axum, tower, validation, CORS |
| 8 | Web | [Database Layer](projects/web-2/README.md) | sqlx, PostgreSQL, migrations, transactions |
| 9 | Web | [Auth & Authorization](projects/web-3/README.md) | JWT, argon2, RBAC |
| 10 | Web | [gRPC and GraphQL](projects/web-4/README.md) | tonic, async-graphql, dataloaders |
| 11 | Web | [Schema Migrations & Zero-Downtime](projects/web-5/README.md) | Expand/contract, backfill, concurrent indexes |
| 12 | Production | [Observability Stack](projects/prod-1/README.md) | tracing, Prometheus, health checks |
| 13 | Production | [Testing Mastery](projects/prod-2/README.md) | proptest, fuzzing, insta, mockall |
| 14 | Production | [Performance & Profiling](projects/prod-3/README.md) | criterion, flamegraphs, caching |
| 15 | Production | [Deployment Pipeline](projects/prod-4/README.md) | Docker, config, graceful shutdown, CI/CD |
| 16 | Production | [Production Debugging](projects/prod-5/README.md) | GDB, flamegraphs, DHAT, strace, tokio-console, Miri |
| 17 | Production | [Chaos Engineering & Failure Recovery](projects/prod-6/README.md) | Circuit breakers, bulkheads, load shedding, degradation |
| 18 | Advanced | [Message Queue Consumer](projects/advanced-1/README.md) | Redis streams, consumer groups, dead letters |
| 19 | Advanced | [Caching & Rate Limiting](projects/advanced-2/README.md) | Multi-tier cache, token bucket, circuit breaker |
| 20 | Advanced | [WebSocket Real-Time](projects/advanced-3/README.md) | WebSockets, DashMap, CRDT |
| 21 | Advanced | [Background Job Processor](projects/advanced-4/README.md) | PostgreSQL job queue, SKIP LOCKED, workers |
| 22 | Distributed | [Raft Consensus](projects/dist-1/README.md) | Leader election, log replication, snapshots |
| 23 | Distributed | [Percolator Transactions](projects/dist-2/README.md) | Snapshot isolation, 2PC, MVCC |
| 24 | Distributed | [Service Mesh](projects/dist-3/README.md) | Service discovery, sagas, distributed tracing |
| 25 | Senior | [API Design & Versioning](projects/senior-1/README.md) | OpenAPI, utoipa, backward compatibility |
| 26 | Senior | [Security Hardening](projects/senior-2/README.md) | OWASP, sanitization, HMAC, audit logging |
| 27 | Senior | [System Design & Interview](projects/senior-3/README.md) | Design docs, capacity estimation, code review |
| 28 | Senior | [Unsafe Rust & FFI](projects/senior-4/README.md) | FFI bindings, allocators, Pin, lock-free, Miri |
| 29 | Capstone | [Production Backend Platform](projects/capstone/README.md) | Everything combined |
| 30 | Open Source | [First Open Source Contribution](projects/oss-1/README.md) | Finding projects, submitting PRs |
| 31 | Open Source | [Publish a Crate](projects/oss-2/README.md) | API design, docs, crates.io, CI |
| 32 | Portfolio | [Portfolio Showcase](projects/portfolio-1/README.md) | GitHub profile, READMEs, blog post, deploy |
| 33 | Portfolio | [Collaborative Development Simulation](projects/collab-1/README.md) | RFCs, code review, sprints, git workflow |
| 34 | Interview | [Technical Interview Gauntlet](projects/interview-1/README.md) | Rust problems, system design, code review |
| 35 | Interview | [Behavioral & Remote Interview Prep](projects/interview-2/README.md) | STAR stories, take-home, mock interviews |
| 36 | Job Search | [Job Search Sprint](projects/job-search-1/README.md) | Applications, networking, offer negotiation |


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
