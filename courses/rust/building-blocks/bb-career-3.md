# Building Block: Salary Negotiation, Offer Evaluation, and Equity

Getting an offer is only half the battle. Understanding what the offer is worth
and negotiating effectively can mean tens of thousands of dollars in difference.
This is especially true for remote roles where salary bands vary widely by
company and geography.

## Readings

- [Patrick McKenzie: Salary Negotiation](https://www.kalzumeus.com/2012/01/23/salary-negotiation/)
  — the single most important article on this topic. Read it twice.
- [levels.fyi](https://www.levels.fyi/) — look up Rust engineer compensation at
  companies you're interested in. Filter by remote.
- [Haseeb Qureshi: Ten Rules for Negotiating a Job Offer](https://haseebq.com/my-ten-rules-for-negotiating-a-job-offer/)
  — practical, step-by-step negotiation tactics.

## Key Concepts

**Understanding total compensation:**

```
Total Comp = Base Salary + Equity/RSUs + Bonus + Benefits

Example:
  Base:    $150,000/year
  Equity:  $40,000/year (vesting over 4 years)
  Bonus:   $15,000/year (performance-dependent)
  Benefits: health insurance, 401k match, equipment stipend
  ─────────────────────────
  Total:   ~$205,000/year
```

**Remote salary models you'll encounter:**

1. **Location-based**: Pay adjusted for your city's cost of living. Common at
   large companies (GitLab, Buffer publish their formulas).
2. **Location-agnostic**: Same pay regardless of where you live. Less common
   but growing. Usually at startups or companies with strong remote culture.
3. **Band-based**: Tiers like "US", "Europe", "Rest of World" with different
   ranges per tier.

**Equity basics for startups:**

- **Stock options (ISOs/NSOs)**: The right to buy shares at a set price. Worth
  nothing if the company never has a liquidity event (IPO or acquisition).
- **RSUs**: Actual shares that vest over time. More common at public companies.
- **Vesting schedule**: Typically 4 years with a 1-year cliff. You get nothing
  if you leave before the cliff.
- **Key question to ask**: "What was the last 409A valuation, and what
  percentage of the company do my options represent?" Most startups won't tell
  you the percentage — that's a yellow flag.
- **Rule of thumb**: Value startup equity at $0 when comparing offers. If the
  startup succeeds, it's a bonus. Don't take a below-market salary for equity
  alone.

**Negotiation principles:**

1. **Never give a number first.** If asked "What's your salary expectation?"
   respond: "I'd like to learn more about the role and the full compensation
   package before discussing numbers." If pressed: "I'm targeting competitive
   market rate for this role and experience level."
2. **Always negotiate.** Companies expect it. Not negotiating signals
   inexperience. Even 5 minutes of discomfort can mean $10K+ per year.
3. **Negotiate base salary first, then equity, then everything else.** Base
   compounds — a $10K higher base is worth $100K+ over 10 years with raises.
4. **Get it in writing.** Verbal offers mean nothing. Don't resign your current
   job until you have a signed offer letter.
5. **Have a BATNA.** Best Alternative to Negotiated Agreement. Having another
   offer (or being willing to walk away) is the strongest negotiating position.

**Red flags in offers:**

- "We'll adjust your salary after 6 months based on performance" — unlikely.
- Unusually long probation periods (6+ months).
- No equity refresh grants for early employees.
- Vague on benefits or PTO policy.
- Pressuring you to decide within 24-48 hours. A week is standard.

## Exercises

1. Research salary ranges for "Rust Backend Engineer" on levels.fyi, Glassdoor,
   and rustjobs.dev. Create a table with: company, role, location policy, salary
   range, equity range. Note the median.

2. Practice the salary negotiation out loud: a friend asks "What's your salary
   expectation?" and you deflect, then they make an offer $15K below your target
   and you counter. Do this 3 times until it feels natural.

3. Draft an email accepting a hypothetical offer with a counter on base salary.
   Keep it professional, positive, and specific. Under 150 words.
