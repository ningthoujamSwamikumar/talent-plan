# Building Block: Resume and LinkedIn for Rust Engineers

Most Rust job postings receive fewer applicants than Python or JavaScript roles,
but the bar is higher. Your resume needs to demonstrate systems thinking, not
just language knowledge. This building block covers how to present your skills
for Rust backend positions specifically.

## Readings

- [The Tech Resume Inside Out](https://thetechresume.com/) — read the free
  summary. The key insight: resumes are scanned in 6 seconds, so lead with
  impact.
- [Rust Jobs Board](https://rustjobs.dev/) — read 10 real Rust job postings.
  Note the exact phrases they use. Your resume should mirror this language.
- [How Hiring Managers Read Resumes](https://www.kalzumeus.com/2011/10/28/dont-call-yourself-a-programmer/)
  — Patrick McKenzie's classic essay. Read the whole thing.

## Key Concepts

**Resume structure for Rust backend engineers:**

```
[Name]
[Email] | [GitHub] | [LinkedIn] | [Blog/Website if you have one]
[City, Country] | Open to remote

SUMMARY (2-3 sentences)
Backend engineer specializing in Rust. Built production systems including
[capstone brief], [published crate], and [open source contribution]. Focus
on [distributed systems / high-performance APIs / whatever your strength is].

TECHNICAL SKILLS
Languages: Rust (primary), [others]
Frameworks: axum, tokio, tonic, tower, sqlx
Databases: PostgreSQL, Redis
Infrastructure: Docker, GitHub Actions CI/CD, Prometheus, Grafana
Patterns: REST, gRPC, GraphQL, WebSockets, message queues, Raft consensus

PROJECTS (most recent first)
[Capstone Name] — Production Backend Platform
- Built a real-time collaborative platform with REST/WebSocket APIs serving
  [N] concurrent connections
- Implemented JWT auth, RBAC, PostgreSQL with migrations, Redis caching,
  and background job processing
- Deployed with Docker, observability (tracing + Prometheus), and graceful
  shutdown
- [GitHub link]

[Published Crate Name] — Open Source Library
- Published to crates.io with [N] downloads
- [What it does and what problem it solves]
- [GitHub link]

[Open Source Contribution]
- Contributed [what] to [project name] ([PR link])
- [Impact: fixed bug affecting N users, improved performance by X%, etc.]

EXPERIENCE (if any — even non-Rust experience counts)
[Job Title] at [Company] — [Dates]
- [Impact-focused bullets, not task descriptions]

EDUCATION (brief)
```

**Key principles:**

1. **Lead with projects, not education** — for career changers, your course
   projects ARE your experience. Put them first.
2. **Quantify everything** — "handles 10K concurrent connections" not "handles
   many connections." "Reduced response time by 40%" not "improved performance."
3. **Mirror the job posting** — if they say "tokio" say "tokio", not "async
   runtime." If they say "microservices" use that word.
4. **One page maximum** — no exceptions until you have 10+ years of experience.
5. **No "familiar with" or "exposure to"** — either you can do it or leave it
   off. You built these systems. Say so.

**LinkedIn optimization:**

- Headline: "Rust Backend Engineer | Systems Programming | Distributed Systems"
  — not your current job title if you're transitioning.
- About section: 3-4 sentences about what you build and what you're looking for.
- Featured section: pin your capstone repo, published crate, and blog post.
- Skills: add Rust, tokio, PostgreSQL, gRPC, Docker, distributed systems.
  Get endorsements from course peers if possible.

## Exercises

1. Read 10 Rust backend job postings. Create a spreadsheet with columns:
   company, required skills, nice-to-have skills, years of experience, remote
   policy. Identify the 5 most commonly requested skills.

2. Write a first draft of your resume using the template above. Fill in every
   section with real content from your course projects.

3. Update your LinkedIn profile: headline, about section, skills, and featured
   projects. Connect with 10 people in the Rust community.
