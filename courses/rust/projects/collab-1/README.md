# Project: Collaborative Development Simulation

**Phase 9 — Portfolio & Professional Presence**

Real engineering is collaborative. Code reviews, RFCs, merge conflicts, sprint
planning — you'll do this daily as a remote engineer. This project simulates a
full development cycle with team dynamics, even if you're doing it solo.

If you're taking this course with a partner or study group, do this project
together. If you're solo, you'll simulate the multi-person workflow yourself
using multiple git identities or branches.

## Deliverables

By the end of this project you will have:

- [ ] A repository with realistic collaborative git history
- [ ] At least 1 written RFC/design doc with feedback comments
- [ ] At least 3 pull requests with code review comments
- [ ] At least 1 resolved merge conflict
- [ ] A project board (GitHub Projects) with a completed sprint

## Part 1: Set Up the Project

Create a new repository for a small but real project. This is NOT a course
exercise — build something you'd actually use or that solves a real problem.

**Suggested project ideas (pick one):**

- A CLI tool that does something useful (log analyzer, config validator, API
  client for a service you use)
- A small web API for something practical (bookmark manager, expense tracker,
  reading list API)
- A utility library that extends one of your course projects

**Set up the repository with professional infrastructure:**

1. Create the repo with README, LICENSE, and .gitignore
2. Set up GitHub Actions CI (test, clippy, fmt)
3. Create a GitHub Projects board with columns: Backlog, In Progress, Review, Done
4. Enable branch protection on `main`: require PR reviews, require CI to pass
5. Create issue templates (bug report, feature request)

## Part 2: Write an RFC

Before writing code, write a design document for your project. Use the RFC
template from the building block:

```markdown
# RFC: [Project Name]

## Context
What problem does this project solve?

## Goals
- What it will do (3-5 bullets)

## Non-Goals
- What it explicitly won't do

## Technical Design
- Architecture overview
- Data model
- API design (if applicable)
- Error handling strategy
- Testing strategy

## Alternatives Considered
At least 2 alternative approaches you rejected, with reasoning.

## Open Questions
Things you're not sure about yet.
```

**If doing this with a partner:**
Post the RFC as a GitHub issue or discussion. Have your partner review it and
leave at least 5 comments (questions, suggestions, concerns). Respond to each
comment. Iterate until you reach agreement.

**If solo:**
Write the RFC, then switch perspective. Re-read it as a skeptical reviewer.
Leave comments on your own RFC poking holes in the design. Then respond to
those comments.

## Part 3: Sprint Planning

Create 8-12 GitHub issues for the work needed to implement your RFC. Each issue
should be:

- Small enough to complete in 1-3 hours
- Clearly scoped with acceptance criteria
- Labeled (feature, bug, documentation, testing)
- Estimated (small, medium, large)

Assign issues to a "Sprint 1" milestone. Plan for 5-6 issues in the sprint.

## Part 4: Development with PRs and Code Review

Implement the sprint using pull requests:

1. **For each issue**, create a feature branch and implement the change.

2. **Write a proper PR description** for every PR:
   ```markdown
   ## What
   Brief description of the change.

   ## Why
   Link to the issue. Context on the approach.

   ## How to Test
   Steps a reviewer should take to verify this works.

   ## Screenshots/Output
   If applicable.
   ```

3. **Do code reviews on your own PRs** (or have your partner review):
   - Leave at least 2 comments per PR
   - Include at least one "nit" and one substantive comment
   - Request at least one change before approving
   - Use the proper GitHub review workflow (Request Changes → address → Approve → Merge)

4. **Create at least one intentional merge conflict** and resolve it:
   - Have two branches modify the same lines of a file
   - Merge one first
   - Resolve the conflict on the second PR
   - Document how you resolved it in the PR description

## Part 5: Mid-Sprint Course Correction

Halfway through the sprint, write a brief status update as if you were posting
to a team Slack channel:

```markdown
## Sprint 1 — Mid-Sprint Update

**Completed:** [list]
**In progress:** [list]
**Blocked:** [anything blocking you, and what you need to unblock]
**At risk:** [anything that might not make the sprint]
**Plan for the rest of the sprint:** [priorities]
```

If you realize the sprint scope was wrong (too much or too little), adjust.
Move issues out of the sprint or pull new ones in. Document why.

## Part 6: Sprint Retrospective

After completing the sprint, write a retrospective:

1. **What went well?** — What processes or approaches worked?
2. **What didn't go well?** — What was harder than expected? What took too long?
3. **What will you do differently?** — Concrete changes for next time.
4. **Metrics:**
   - Issues planned: X
   - Issues completed: Y
   - Issues carried over: Z
   - Total PRs: N
   - Average PR review time: (even if self-reviewed)

Post this as a GitHub Discussion or a file in the repo.

## Success Criteria

- Repository has a realistic git history (feature branches, PRs, merge commits)
- RFC exists with review comments and iterations
- At least 3 PRs were submitted with code review comments
- At least 1 merge conflict was resolved
- GitHub Projects board shows completed sprint
- Sprint retrospective is written

## Why This Matters

When an interviewer asks "Tell me about your experience working on a team" or
"How do you handle code reviews?", you can point to this repository and walk
them through a real development cycle. This is infinitely more convincing than
saying "I've read about agile."
