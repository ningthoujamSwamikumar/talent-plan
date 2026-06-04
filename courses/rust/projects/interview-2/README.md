# Project: Behavioral & Remote Interview Prep

**Phase 10 — Interview Preparation**

Technical skills get you the interview. This project prepares you for everything
else: behavioral questions, take-home assignments, and the soft-skill evaluation
that determines whether you get the offer.

Remote Rust roles have a particular interview pattern: initial recruiter screen,
take-home coding challenge, technical deep dive, system design, behavioral/culture
fit. This project prepares you for the non-coding stages.

## Deliverables

By the end of this project you will have:

- [ ] 10 polished STAR stories written and practiced
- [ ] A completed take-home assignment (timed, 4-6 hours)
- [ ] A "Questions for the interviewer" document
- [ ] A post-interview follow-up email template
- [ ] At least 2 mock behavioral interviews (recorded or with a partner)

---

## Part 1: Write Your STAR Stories

Write 10 stories using the STAR format (Situation, Task, Action, Result). Each
should be 150-250 words and deliverable in under 2 minutes when spoken aloud.

**Required stories (draw from your course experience):**

| # | Question | Suggested Source |
|---|----------|-----------------|
| 1 | "Tell me about a difficult technical problem you solved" | Raft consensus edge cases, or a debugging session that required deep investigation |
| 2 | "Tell me about a time you made a design tradeoff" | Storage engine choice (Phase 1), database schema decisions, sync vs async |
| 3 | "Tell me about a time you improved an existing system" | Performance optimization (Phase 3), or refactoring for testability |
| 4 | "How do you approach an unfamiliar codebase?" | Your open source contribution (Phase 8) |
| 5 | "Tell me about a project you're proud of" | Your capstone project |
| 6 | "Tell me about a time something didn't go as planned" | A project where your initial approach failed and you had to pivot |
| 7 | "How do you handle disagreements in code reviews?" | Your collaborative development simulation (Phase 9) |
| 8 | "Tell me about a time you had to learn something quickly" | Picking up a new technology during the course (async, gRPC, Raft, etc.) |
| 9 | "How do you prioritize when you have too much to do?" | Sprint planning and scope adjustment in the collaboration project |
| 10 | "Tell me about a time you helped someone else" | Answering questions on Rust Discord/forum, or mentoring a study partner |

**For each story, also prepare:**
- "What would you do differently?" — shows reflection and growth
- A 30-second version — sometimes interviewers want brevity
- A follow-up technical detail — if they ask "can you go deeper on the technical
  side?"

## Part 2: Take-Home Assignment

Simulate a real take-home coding challenge. This is common for remote Rust roles.

**The assignment:**

Build a REST API for a simple service in 4-6 hours (set a timer). Requirements:

- Implement a **bookmark manager API** with these endpoints:
  - `POST /bookmarks` — create a bookmark (url, title, tags, notes)
  - `GET /bookmarks` — list bookmarks with pagination and tag filtering
  - `GET /bookmarks/:id` — get a single bookmark
  - `PUT /bookmarks/:id` — update a bookmark
  - `DELETE /bookmarks/:id` — delete a bookmark
  - `GET /bookmarks/search?q=` — full-text search across title and notes

- Technical requirements:
  - Use axum and PostgreSQL (sqlx)
  - Include input validation
  - Include error handling with appropriate HTTP status codes
  - Include at least 10 tests (unit + integration)
  - Include a README with setup instructions and API documentation
  - Include a Dockerfile

- Evaluation criteria (this is what real take-homes are graded on):
  - **Correctness**: Does it work? Do the tests pass?
  - **Code quality**: Is it idiomatic Rust? Is it readable?
  - **Error handling**: Are errors handled gracefully?
  - **Testing**: Are the tests meaningful (not just happy path)?
  - **Documentation**: Can someone else run and understand this?
  - **Time management**: Did you ship something complete in the time box?

**After the timer:**
- Stop coding. Whatever you have is your submission.
- Write a "What I would do with more time" section in the README. This shows
  maturity and prioritization skills.
- Commit everything and push. Review your own code as if you were the
  interviewer.

## Part 3: Questions for the Interviewer

Having thoughtful questions shows you're evaluating the company, not just hoping
they'll hire you. Prepare 15+ questions across categories, then pick 3-5 per
interview round.

**About the team and role:**
1. What does a typical day/week look like for this role?
2. How is the team structured? How many backend engineers?
3. What's the on-call rotation like?
4. What's the ratio of new feature work vs. maintenance vs. tech debt?
5. How do you handle production incidents?

**About engineering culture:**
6. What does your code review process look like?
7. How do you make architectural decisions? RFCs? ADRs?
8. What's your testing philosophy? What's the test coverage like?
9. How do you handle technical disagreements?
10. What's the most interesting technical challenge the team solved recently?

**About Rust specifically:**
11. How long has the team been using Rust? What drove the decision?
12. What's the Rust version policy? (stable only? nightly features?)
13. What crates are core to the stack?
14. Are there parts of the codebase that use `unsafe`? How is it reviewed?
15. How do you handle Rust compile times in CI?

**Red-flag detection questions:**
16. Why is this position open?
17. What happened to the last person in this role?
18. What's the company's runway / financial position? (for startups)
19. How do you measure engineering productivity?
20. What's something about the engineering culture you'd like to change?

## Part 4: Post-Interview Communication

**Thank-you email template (send within 24 hours):**

```
Subject: Thank you — [Role Title] interview

Hi [Name],

Thank you for taking the time to speak with me today about the [Role Title]
position. I enjoyed our conversation about [specific topic you discussed —
shows you were paying attention].

I'm particularly excited about [something specific about the role or company
that genuinely interests you]. The [technical challenge / team structure /
product direction] aligns well with my experience building [specific project
from your portfolio].

[If applicable: I wanted to follow up on [question that came up during the
interview]. After thinking about it more, I believe [your refined answer].]

I look forward to the next steps. Please don't hesitate to reach out if you
need any additional information.

Best regards,
[Your name]
```

**Key principles:**
- Send within 24 hours. Same day is ideal.
- Reference something specific from the conversation.
- Keep it under 150 words.
- No desperation. No "I really hope to hear from you."
- If you blanked on a question during the interview, this is your chance to
  provide a better answer.

## Part 5: Mock Behavioral Interviews

Do at least 2 mock behavioral interviews:

**Round 1: Remote work focus (20 minutes)**
Have someone ask you (or practice solo with a recording):
1. "How do you structure your workday when working remotely?"
2. "How do you communicate when you're blocked on something?"
3. "Tell me about a time you had a miscommunication with a remote teammate."
4. "How do you stay connected with your team across time zones?"

**Round 2: Engineering depth (20 minutes)**
1. "Walk me through the architecture of a system you've built."
2. "Tell me about a performance problem you diagnosed and fixed."
3. "How do you decide when to use Rust vs. another language?"
4. "What's a technical opinion you hold that most people disagree with?"

**After each round:**
- Review the recording or get feedback from your partner
- Note where you rambled, went silent, or didn't answer the actual question
- Rewrite any weak answers
- Practice again until each answer is crisp and under 2 minutes

## Success Criteria

- 10 STAR stories written, each 150-250 words
- Take-home assignment completed within the time box
- At least 15 interviewer questions prepared across categories
- Thank-you email template written
- 2 mock behavioral interviews completed with self-review notes
