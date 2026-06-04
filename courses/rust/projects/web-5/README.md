# Project: Schema Migrations & Zero-Downtime Deploys

**Phase 2 — Backend Web Development**

In production, you can never take the database offline to change the schema.
You deploy new code while old code is still running. You add columns while
millions of rows exist. You rename fields while other services still read the
old name. This project teaches the migration patterns that every production
backend engineer must know.

## Prerequisites

- Completion of web-2 (Database Layer) and web-3 (Auth & Authorization)
- Building block bb-schema (Schema Migrations)
- A running PostgreSQL instance

## Deliverables

- [ ] A multi-step schema migration executed across 4 deploys
- [ ] A batch backfill job that processes rows without locking the table
- [ ] Concurrent index creation without blocking writes
- [ ] A migration rollback executed safely
- [ ] Tests verifying backwards compatibility at each migration step

---

## Part 1: The Starting Schema

You are working with a `tasks` table that's been in production for months:

```sql
CREATE TABLE tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    assigned_to VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

This table has 100,000 rows (seed them in a setup script). Old code reads and
writes all these columns. Your goal: evolve this schema through 4 migration
steps while keeping the service running.

**Seed script:** Write a Rust function that inserts 100,000 tasks with realistic
data (random names, descriptions, statuses, assigned users, timestamps spread
over 6 months). Use `sqlx` batch inserts for speed.

## Part 2: Migration 1 — Split `name` Into `title` + `slug`

**Goal:** Replace `name VARCHAR(255)` with `title VARCHAR(500)` and
`slug VARCHAR(255) UNIQUE`.

**Step 2a: Expand — Add new columns (Migration file 001)**

```sql
ALTER TABLE tasks ADD COLUMN title VARCHAR(500);
ALTER TABLE tasks ADD COLUMN slug VARCHAR(255);
```

These are nullable — old code keeps working because it doesn't read these
columns. No lock beyond brief `ALTER TABLE` (adding nullable column is fast).

**Step 2b: Dual-write — Update application code**

Deploy code that:
- On INSERT: writes to both `name` AND `title`/`slug`
- On UPDATE: updates both `name` AND `title`/`slug`
- On SELECT: reads from `name` (not yet from `title`/`slug`)

Write a test that creates a task via the new code and verifies both `name` and
`title`/`slug` are populated.

**Step 2c: Backfill — Populate existing rows**

Implement a backfill job in `src/backfill.rs`:

```rust
pub async fn backfill_title_slug(pool: &PgPool, batch_size: i64) -> Result<u64> {
    let mut total = 0;
    loop {
        let affected = sqlx::query!(
            r#"
            UPDATE tasks
            SET title = name, slug = lower(replace(name, ' ', '-'))
            WHERE title IS NULL
            AND id IN (
                SELECT id FROM tasks WHERE title IS NULL
                ORDER BY id LIMIT $1
            )
            "#,
            batch_size
        )
        .execute(pool)
        .await?
        .rows_affected();

        total += affected;
        if affected == 0 { break; }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    Ok(total)
}
```

**Tests:**
- `backfill_processes_all_rows` — all 100K rows have `title` and `slug` after
- `backfill_is_idempotent` — running twice doesn't break anything
- `backfill_handles_concurrent_writes` — new rows created during backfill are
  handled (they already have `title`/`slug` from dual-write)

**Step 2d: Switch readers**

Deploy code that:
- On SELECT: reads from `title`/`slug` instead of `name`
- On INSERT/UPDATE: still writes to both (for rollback safety)

**Step 2e: Contract — Add constraints and drop old column (Migration file 002)**

```sql
ALTER TABLE tasks ALTER COLUMN title SET NOT NULL;
-- Create unique index concurrently (doesn't lock table)
CREATE UNIQUE INDEX CONCURRENTLY idx_tasks_slug ON tasks(slug);
-- Only after verifying everything works:
ALTER TABLE tasks DROP COLUMN name;
```

**Test:** Verify that old `name` column is gone and all reads/writes use
`title`/`slug`.

## Part 3: Migration 2 — Add `priority` with Constraint

**Goal:** Add a `priority` column with a CHECK constraint, without locking the
table during constraint validation.

**Step 3a: Add column with default (Migration file 003)**

```sql
ALTER TABLE tasks ADD COLUMN priority SMALLINT DEFAULT 3;
```

This is fast (no rewrite in PostgreSQL 11+).

**Step 3b: Add constraint as NOT VALID (Migration file 004)**

```sql
ALTER TABLE tasks ADD CONSTRAINT tasks_priority_range
    CHECK (priority BETWEEN 1 AND 5) NOT VALID;
```

`NOT VALID` means PostgreSQL enforces the constraint on new rows immediately
but does NOT scan existing rows. No long lock.

**Step 3c: Validate the constraint (Migration file 005)**

```sql
ALTER TABLE tasks VALIDATE CONSTRAINT tasks_priority_range;
```

This scans the table but only acquires a `SHARE UPDATE EXCLUSIVE` lock (allows
reads and writes, blocks only schema changes). Safe in production.

**Test:** Verify that inserting `priority = 0` or `priority = 6` fails, while
`priority = 3` succeeds.

## Part 4: Migration 3 — Add a GIN Index for Full-Text Search

**Goal:** Add a full-text search index on `title` and `description` without
blocking writes.

```sql
-- This would LOCK the table:
-- CREATE INDEX idx_tasks_search ON tasks USING GIN (to_tsvector('english', title || ' ' || coalesce(description, '')));

-- This does NOT lock the table:
CREATE INDEX CONCURRENTLY idx_tasks_search ON tasks
    USING GIN (to_tsvector('english', title || ' ' || coalesce(description, '')));
```

**What if `CONCURRENTLY` fails?** The index is left in an `INVALID` state.
You must drop it and retry:
```sql
DROP INDEX IF EXISTS idx_tasks_search;
-- then retry CREATE INDEX CONCURRENTLY
```

Implement a migration runner that detects failed concurrent indexes and retries.

**Test:** Verify that full-text search queries use the index (check
`EXPLAIN ANALYZE` output for `Bitmap Index Scan`).

## Part 5: Rollback a Migration

**Scenario:** You deployed migration 003 (priority column) but discovered a
bug — the default value should be 2, not 3.

**Rollback steps:**
1. Deploy code that doesn't depend on `priority` having a specific default
2. Run rollback migration:
   ```sql
   ALTER TABLE tasks ALTER COLUMN priority SET DEFAULT 2;
   UPDATE tasks SET priority = 2 WHERE priority = 3 AND updated_at > '(deploy_time)';
   ```
3. Verify the rollback didn't break any tests

**Test:** Verify that tasks created after the rollback have `priority = 2` as
default.

## Part 6: Migration Test Framework

Build a reusable migration test harness in `tests/migration_tests.rs`:

```rust
async fn test_migration_step(pool: &PgPool, migration: &str, assertions: impl Fn(&PgPool)) {
    // 1. Run the migration
    // 2. Run assertions
    // 3. Verify old code still works (backwards compatibility)
}
```

Write a full integration test that:
1. Seeds 100K rows
2. Runs all 5 migrations in order
3. At each step, verifies:
   - Old API still works (backwards compat)
   - New API works
   - Data integrity is maintained
4. Runs the rollback
5. Verifies the rollback is clean

## Testing

```
cargo test
```

Requires a running PostgreSQL instance. Set `DATABASE_URL` in `.env`.

## What You Will Learn

- The expand/contract pattern for zero-downtime schema changes
- How to write batch backfills that don't lock production tables
- When and how to use `CREATE INDEX CONCURRENTLY`
- How to add constraints without scanning the entire table
- How to rollback migrations safely
- How to test migrations with backwards compatibility assertions
- Which PostgreSQL DDL operations lock tables and which don't
