# Building Block: GitHub Profile, Portfolio, and Professional Presence

Your GitHub profile is your technical resume for remote jobs. Hiring managers for
remote Rust positions will look at your GitHub before they look at your PDF
resume. This building block covers how to present your work so it tells a
compelling story.

## Readings

- [GitHub profile README guide](https://docs.github.com/en/account-and-profile/setting-up-and-managing-your-github-profile/customizing-your-profile/managing-your-profile-readme)
  — how to create and customize your profile README.
- [How to write a good README](https://www.makeareadme.com/) — the basics.
- [Awesome README examples](https://github.com/matiassingers/awesome-readme)
  — study what makes these stand out.
- [Writing a technical blog post](https://jvns.ca/blog/2023/06/05/some-blogging-myths/)
  — Julia Evans on why and how to blog about technical work.

## Key Concepts

**Your GitHub profile should communicate three things:**

1. **What you build** — pinned repositories showing range (systems, web, distributed)
2. **How you build** — clean code, tests, documentation, CI
3. **That you're active** — recent commits, contributions, published crates

**Which projects to showcase (pin these repos):**

- Your **capstone** project — the flagship. It should have the best README.
- Your **published crate** — shows you can ship reusable software.
- **One systems project** (Phase 1) — shows low-level Rust skills.
- **One contribution to a well-known project** — shows you work with real teams.

**What a great project README contains:**

```
# Project Name
One-sentence description of what it does and why.

## Features
- Bullet list of key capabilities

## Architecture
Brief description or diagram of how the system works.

## Getting Started
```sh
git clone ...
cargo run
```

## Usage
Code examples showing the main use cases.

## Testing
How to run the test suite, what it covers.

## Performance
Benchmarks or profiling results if relevant.

## License
```

**Writing your first technical blog post:**

You don't need a fancy blog. A GitHub gist, dev.to post, or simple static site
works. Write about ONE specific thing you learned or built:

- "How I implemented Raft consensus in Rust — and what surprised me"
- "Building a zero-copy rate limiter with tower middleware"
- "Lessons from publishing my first Rust crate"

Keep it under 1500 words. Include code snippets. Be honest about what was hard.

## Exercises

1. Find 3 GitHub profiles of employed Rust engineers (check contributors on
   tokio, axum, or other major projects). Study their profiles: what repos are
   pinned? How are READMEs written? Do they have a profile README?

2. Draft a profile README for yourself. Include: who you are, what you're
   building, your key skills, and links to your best work. Keep it under 20
   lines.

3. Write a README outline for your capstone project following the template above.
   Focus on the Architecture section — this is what distinguishes a senior
   engineer's documentation from a beginner's.
