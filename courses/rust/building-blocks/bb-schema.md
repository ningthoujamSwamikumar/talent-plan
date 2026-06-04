# Building Block: Schema Migrations and Zero-Downtime Deploys

In production, you can't just `DROP TABLE` and start over. Every schema change
must happen while the service is running, while other services depend on the
current schema, and while real users are making real requests. This building
block covers the patterns that make zero-downtime schema evolution possible.

## Readings

- [Safe database migrations at scale (Stripe)](https://stripe.com/blog/online-migrations)
  — how Stripe migrates databases with zero downtime. The gold standard.
- [SQLx migrations documentation](https://docs.rs/sqlx/latest/sqlx/migrate/index.html)
  — how SQLx handles migration files.
- [Expand/Contract pattern](https://www.tim-wellhausen.de/papers/ExpandAndContract.html)
  — the fundamental pattern for safe schema evolution.
- [PostgreSQL lock levels](https://www.postgresql.org/docs/current/explicit-locking.html)
  — which DDL statements take which locks. Critical for understanding what
  blocks production traffic.
- [Strong Migrations (Ruby, but concepts transfer)](https://github.com/ankane/strong_migrations)
  — a catalog of dangerous migration patterns and their safe alternatives.

## Key Concepts

**The expand/contract pattern:**

Every schema change follows three phases:

```
Phase 1: EXPAND
  Add new column/table alongside existing ones.
  Both old and new code work.

Phase 2: MIGRATE
  Deploy new code that writes to both old and new locations.
  Backfill existing data.

Phase 3: CONTRACT
  Deploy code that reads only from new location.
  Remove old column/table.
```

Never do all three in one deployment. Each phase is a separate deploy,
potentially days or weeks apart.

**Dangerous migration patterns:**

| Pattern | Danger | Safe Alternative |
|---------|--------|------------------|
| `ALTER TABLE ADD COLUMN NOT NULL` | Locks table, rewrites all rows | Add nullable column, backfill, then add constraint |
| `ALTER TABLE DROP COLUMN` | Old code that reads this column crashes | Deploy code that ignores column first, then drop |
| `CREATE INDEX` | Locks table for writes | `CREATE INDEX CONCURRENTLY` |
| `ALTER TABLE ADD CONSTRAINT` | Scans entire table, blocks writes | `ADD CONSTRAINT ... NOT VALID`, then `VALIDATE CONSTRAINT` |
| `ALTER TABLE RENAME COLUMN` | Breaks all code referencing old name | Add new column, copy data, update code, drop old |
| `ALTER TABLE ALTER TYPE` | Rewrites entire table | Add new column with new type, migrate, drop old |

**Backfill strategies:**

Large table backfills must be done in batches to avoid:
- Locking the table for extended periods
- Bloating the WAL (write-ahead log)
- Causing replication lag

```sql
-- Batch backfill pattern
UPDATE tasks
SET new_column = compute_value(old_column)
WHERE id IN (
    SELECT id FROM tasks
    WHERE new_column IS NULL
    ORDER BY id
    LIMIT 1000
)
```

Run this in a loop with a sleep between batches. Monitor replication lag.

**Rollback strategies:**

Every migration should have a rollback plan:
- **Additive changes** (new column, new table): rollback = drop the new thing
- **Data transformations**: rollback = keep the old column until contract phase
- **Destructive changes** (drop column): no rollback possible — this is why you
  contract last, after verifying everything works

## Exercises

1. Plan a migration for this scenario: You have a `users` table with a `name`
   column (VARCHAR). You need to split it into `first_name` and `last_name`.
   Write out the expand/contract steps with the exact SQL for each step and
   the code changes required between steps.

2. Write a SQL migration that adds an index to a 10M-row table without locking
   it. What PostgreSQL command do you use? What happens if the index creation
   fails?

3. Write a batch backfill script in Rust using sqlx. It should: process 1000
   rows at a time, log progress, handle errors gracefully, and be resumable
   (if interrupted, it picks up where it left off).

---

## Checklist

- [ ] You can explain the expand/contract pattern
- [ ] You know which DDL statements lock tables and which don't
- [ ] You can plan a multi-step migration with rollback at each step
- [ ] You can write a batch backfill that doesn't lock the table
- [ ] You understand `CREATE INDEX CONCURRENTLY` and its failure modes
- [ ] You can identify dangerous migration patterns and suggest safe alternatives

Next: [Project: Schema Migrations & Zero-Downtime Deploys](../projects/web-5/README.md)
