# Building Block Web-2: PostgreSQL and SQLx

**Prerequisites**: [Project: REST API with Axum](../projects/web-1/README.md).

Before starting [Project: Database Layer](../projects/web-2/README.md), complete
the readings and exercises below.

## What to read

- [SQLx documentation](https://docs.rs/sqlx/latest/sqlx/).
  Focus on the `query!` and `query_as!` macros, `PgPool`, and the `FromRow` derive.

- [SQLx GitHub README](https://github.com/launchbadge/sqlx).
  The README has excellent examples covering migrations, compile-time checking,
  and offline mode.

- [PostgreSQL Tutorial](https://www.postgresqltutorial.com/).
  If you're new to PostgreSQL, work through the basics: CREATE TABLE, INSERT,
  SELECT with JOINs, UPDATE, DELETE, indexes, and transactions.

- [PostgreSQL Full-Text Search](https://www.postgresql.org/docs/current/textsearch.html).
  Overview of tsvector, tsquery, and GIN indexes. You'll use this in the project.

- [The Repository Pattern in Rust](https://www.shuttle.rs/blog/2024/02/02/using-traits-with-rust).
  How to structure data access code cleanly.

## Key concepts

### SQLx Compile-Time Checking

```rust
// query_as! checks the SQL against your actual database at compile time
let project = sqlx::query_as!(
    Project,
    "SELECT id, name, description, created_at, updated_at FROM projects WHERE id = $1",
    project_id
)
.fetch_optional(&pool)
.await?;
```

This requires `DATABASE_URL` at compile time. For CI without a database, use
`cargo sqlx prepare` to save query metadata to `.sqlx/` directory.

### Migrations

```bash
# Create a new migration
sqlx migrate add initial_schema

# Run migrations programmatically
sqlx::migrate!().run(&pool).await?;
```

### Connection Pooling

```rust
let pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(3))
    .connect(&database_url)
    .await?;
```

### Transactions

```rust
let mut tx = pool.begin().await?;
sqlx::query!("UPDATE tasks SET project_id = $1 WHERE project_id = $2", new_id, old_id)
    .execute(&mut *tx)
    .await?;
sqlx::query!("DELETE FROM projects WHERE id = $1", old_id)
    .execute(&mut *tx)
    .await?;
tx.commit().await?;
```

## Exercises

**Exercise 1**: Install PostgreSQL and create a database. Connect to it with
`sqlx::PgPool::connect()` and run a simple query.

**Exercise 2**: Write a migration that creates a `users` table and run it
with `sqlx::migrate!()`.

**Exercise 3**: Implement CRUD operations for users using `query_as!`. Test
that compile-time checking catches typos in column names.

**Exercise 4**: Write a transaction that transfers a value between two rows,
and verify it rolls back on error.

## You're ready when...

- [ ] You can connect to PostgreSQL from Rust
- [ ] You can write and run migrations
- [ ] You understand compile-time query checking
- [ ] You can use transactions for multi-step operations
- [ ] You know how to use `cargo sqlx prepare` for offline builds

Next: [Project: Database Layer](../projects/web-2/README.md)
