# Project: Portfolio Showcase

**Phase 9 — Portfolio & Professional Presence**

This project is about presentation, not code. You've built impressive systems
over the course of this program. Now you need to make that work visible and
compelling to hiring managers who will spend 30 seconds on your GitHub profile
before deciding whether to interview you.

## Deliverables

By the end of this project you will have:

- [ ] A GitHub profile README
- [ ] Polished READMEs for 3 course projects (including the capstone)
- [ ] Your capstone deployed with a live URL
- [ ] One published technical blog post
- [ ] 4 pinned repositories on your GitHub profile

## Part 1: GitHub Profile README

Create a repository with the same name as your GitHub username (e.g.,
`github.com/yourname/yourname`). The `README.md` in this repo appears on your
profile page.

**Template:**

```markdown
# Hi, I'm [Name]

Backend engineer specializing in Rust. I build high-performance, production-grade
distributed systems.

## What I've built

- **[Capstone Name]** — Real-time collaborative platform with REST, WebSocket,
  gRPC APIs, PostgreSQL, Redis, background jobs, and full observability stack.
  [Repo](link) | [Live Demo](link)
- **[Crate Name]** — [What it does]. Published on
  [crates.io](https://crates.io/crates/name).
- **Contributor to [Project]** — [Brief description of contribution]. [PR](link)

## Tech

Rust | tokio | axum | sqlx | PostgreSQL | Redis | gRPC | Docker | Prometheus

## Currently

[What you're working on or looking for. e.g., "Looking for remote Rust backend
engineering roles. Open to interesting systems problems."]
```

Keep it under 30 lines. No emojis. No "visitor count" badges. Clean and
professional.

## Part 2: Polish 3 Project READMEs

Choose 3 projects to showcase. One MUST be the capstone. The other two should
show range (e.g., one systems project from Phase 1, one distributed systems
project from Phase 5).

For each, rewrite the README to be a **portfolio piece**, not a course assignment.
Remove references to "this course" or "this project." Write it as if you built
it independently.

**README structure for portfolio projects:**

```markdown
# Project Name

One-paragraph description: what it is, what problem it solves, and the key
technical decisions.

## Architecture

Describe the system design. Include:
- High-level component diagram (text-based is fine: ASCII or Mermaid)
- Key data flows
- Technology choices and WHY you made them

## Features

- Bullet list of what the system does
- Focus on the technically interesting parts

## Technical Highlights

Pick 2-3 things that show senior-level thinking:
- "Uses SKIP LOCKED for distributed job claiming without external coordination"
- "Implements optimistic concurrency control with version vectors"
- "Achieves <5ms p99 latency under 10K req/s (benchmarked with criterion)"

## Getting Started

git clone, cargo run, and any setup steps. A reviewer should be able to run
your project in under 2 minutes.

## Testing

How to run tests. What the test suite covers. Any interesting testing
techniques (property-based, fuzzing, snapshot).

## Performance

Include benchmark results if you have them. Flamegraphs are impressive.

## License
```

**What makes a portfolio README stand out:**

- Architecture diagrams (even simple ASCII ones)
- Concrete numbers (latency, throughput, test count)
- Honest "Limitations" or "Future Work" section — shows maturity
- Screenshots or terminal recordings for CLI/TUI tools

## Part 3: Deploy Your Capstone

A live demo URL is the most compelling thing you can put on a resume. Deploy
your capstone to a free or cheap hosting service:

**Options:**

| Service | Free Tier | Good For |
|---------|-----------|----------|
| [Fly.io](https://fly.io) | 3 shared VMs free | Full backend with PostgreSQL |
| [Railway](https://railway.app) | $5 free credit/month | Quick deploy with database |
| [Render](https://render.com) | Free web services | Simple API deployment |
| [Shuttle](https://shuttle.rs) | Rust-native, free tier | Rust-specific hosting |

**Minimum for the demo:**

1. API is accessible at a public URL
2. Health check endpoint returns 200
3. At least one feature is demonstrable (e.g., create a task, list tasks)
4. Add the live URL to your README and GitHub profile

You don't need to keep it running forever — having it up during your job search
is sufficient. Include a note: "Live demo may be sleeping on free tier — first
request may take 10-15s to wake."

## Part 4: Write a Technical Blog Post

Write one post about something you built or learned in this course. Publish it
on [dev.to](https://dev.to), [hashnode.com](https://hashnode.com), or your own
blog.

**Good topics:**

- "Building a Raft consensus implementation in Rust: lessons learned"
- "How I built a real-time WebSocket server with axum and DashMap"
- "Property-based testing in Rust caught bugs my unit tests missed"
- "Publishing my first Rust crate: a step-by-step experience report"
- "From Python to Rust: what I wish I knew earlier"

**Blog post structure:**

1. **Hook** (2-3 sentences) — What did you build and why should the reader care?
2. **Context** (1 paragraph) — What problem were you solving?
3. **Approach** (3-5 paragraphs) — How did you solve it? Include code snippets.
4. **What went wrong** (1-2 paragraphs) — Honest about challenges. This is the
   most interesting part for readers.
5. **Results** (1 paragraph) — What worked? Performance numbers?
6. **Takeaways** (3-5 bullets) — What would you tell someone starting the same
   project?

**Guidelines:**
- 800-1500 words. Not longer.
- Include 2-4 code snippets (not too long — 10-20 lines each).
- Link to your GitHub repo.
- Share it on r/rust and the Rust users forum after publishing.

## Part 5: Pin Your Best Work

Go to your GitHub profile and pin exactly 4 repositories:

1. **Capstone** — your flagship project
2. **Published crate** — shows you can ship a library
3. **Best course project** — shows technical depth (Raft, service mesh, etc.)
4. **Open source contribution fork** (or the upstream project if they merged it)

Order matters. The first pinned repo is what people see first.

## Success Criteria

- GitHub profile has a clean README with links to your work
- 3 project READMEs are polished and portfolio-ready (no course references)
- Capstone is deployed at a live URL
- Blog post is published and shared
- 4 repos are pinned on your profile
