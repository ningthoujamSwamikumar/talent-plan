# Project: Database Layer with SQLx

**Phase 2, Project 2 -- taskforge-db**

In the previous project (web-1) you built a REST API backed by an in-memory
`HashMap`. That approach is fine for prototyping, but production services need
durable storage that survives restarts, handles concurrent access safely, and
scales beyond a single process. In this project you will replace the in-memory
store with **PostgreSQL**, accessed through the **sqlx** async database driver.

## Why databases matter for backend engineers

Every non-trivial backend stores state. Understanding how to design schemas,
write efficient queries, manage migrations, and handle transactions is
foundational. Even if an ORM handles the details later, knowing what happens
beneath the abstraction lets you debug performance problems, reason about
consistency, and make informed architecture decisions.

## Why sqlx

* **Compile-time checked queries** -- sqlx can verify your SQL against a real
  database at compile time, catching typos and schema mismatches before
  deployment.
* **Async native** -- built on top of tokio, sqlx fits naturally into the same
  async runtime your axum server uses.
* **No DSL overhead** -- you write plain SQL. There is no query builder or
  domain-specific language to learn; the database documentation is your
  reference.

## Why PostgreSQL

PostgreSQL is the most popular open-source relational database. It supports
advanced features you will use in this project: full-text search, generated
columns, check constraints, and sophisticated indexing. Skills you build here
transfer directly to any PostgreSQL deployment, whether self-hosted, on RDS, or
a managed service like Neon or Supabase.

---

## Part 1: Setting Up PostgreSQL

### Install PostgreSQL

Follow the instructions for your platform:

* **macOS**: `brew install postgresql@16 && brew services start postgresql@16`
* **Ubuntu/Debian**: `sudo apt install postgresql && sudo systemctl start postgresql`
* **Docker** (recommended for isolation):
  ```bash
  docker run -d --name taskforge-pg \
    -e POSTGRES_PASSWORD=postgres \
    -p 5432:5432 \
    postgres:16
  ```

### Create the database

```bash
createdb taskforge
# or via psql:
psql -U postgres -c "CREATE DATABASE taskforge;"
```

### Configure the connection string

The connection string follows the libpq format:

```
postgres://user:password@host:port/database
```

Store it in a `.env` file at the project root (see `.env.example`):

```
DATABASE_URL=postgres://postgres:postgres@localhost:5432/taskforge
```

The `dotenvy` crate loads this file automatically at startup so you do not need
to export the variable manually during development.

### The DATABASE_URL environment variable

`sqlx` uses `DATABASE_URL` both at **runtime** (to connect) and at **compile
time** (for query checking). When building without a live database, you will use
"offline mode" (see Part 4 below).

---

## Part 2: Migrations

Database migrations are versioned SQL scripts that evolve your schema over time.
sqlx provides a lightweight migration runner.

### Creating a migration

```bash
cargo install sqlx-cli          # one-time setup
sqlx migrate add initial        # creates migrations/<timestamp>_initial.sql
```

This project already ships with `migrations/001_initial.sql` containing the
schema. Study it carefully:

* **projects** table -- the top-level entity.
* **tasks** table -- belongs to a project via a foreign key (`project_id`
  REFERENCES projects(id) ON DELETE CASCADE).
* **Indexes** on `project_id`, `status`, and `priority` for fast lookups.
* A **generated tsvector column** (`search_vector`) with a GIN index for
  full-text search.

### Running migrations

In code, embed and run migrations at startup:

```rust
sqlx::migrate!().run(&pool).await?;
```

The `migrate!()` macro reads the `migrations/` directory at compile time and
embeds the SQL into the binary. At runtime it applies any unapplied migrations
and records them in a `_sqlx_migrations` table.

---

## Part 3: Repository Pattern

The **repository pattern** separates data-access logic from business logic and
HTTP handlers. Each repository struct owns a `PgPool` and exposes methods like
`create`, `get_by_id`, `list`, `update`, and `delete`.

```
Handler  --->  Repository  --->  PostgreSQL
(HTTP)         (SQL)             (storage)
```

Benefits:

* Handlers stay focused on request/response concerns.
* Repositories can be tested independently (with a test database).
* Swapping the storage backend (e.g., to SQLite for tests) requires changing
  only the repository implementation.

Look at `src/repository/projects.rs` and `src/repository/tasks.rs`. Every
method is stubbed with `todo!()`. Your job is to implement them.

---

## Part 4: CRUD with sqlx

### query vs query_as

* `sqlx::query(sql)` -- returns raw `Row` objects you decode manually.
* `sqlx::query_as::<_, T>(sql)` -- maps rows directly into a struct `T` that
  derives `FromRow`.

Use `query_as` whenever possible for type safety and convenience.

### Bind parameters

Always use bind parameters (`$1`, `$2`, ...) instead of string interpolation to
prevent SQL injection:

```rust
sqlx::query_as::<_, Project>(
    "SELECT * FROM projects WHERE id = $1"
)
.bind(id)
.fetch_optional(&self.pool)
.await
```

### Compile-time checked queries

The `sqlx::query_as!()` macro (note the `!`) checks your SQL against the live
database at compile time. This is powerful but requires `DATABASE_URL` to point
at a running PostgreSQL during `cargo build`.

For CI or environments without a database, use **offline mode**:

```bash
# Generate query metadata (run once with a live DB)
cargo sqlx prepare

# This creates a .sqlx/ directory with JSON files describing each query.
# Commit this directory to version control.
```

With `.sqlx/` present and the `SQLX_OFFLINE=true` environment variable set,
`cargo build` succeeds without a database.

**In this starter project** we use the runtime-checked `query_as::<_, T>()`
form so the code compiles without a live database. Once you have PostgreSQL
running, you are encouraged to migrate to the compile-time `query_as!()` macro
and run `cargo sqlx prepare` to generate offline data.

---

## Part 5: Transactions

Some operations must be atomic -- either every step succeeds or none of them do.
SQL transactions provide this guarantee.

### The exercise: move tasks between projects

`TaskRepository::move_tasks_to_project` must:

1. BEGIN a transaction.
2. Verify both the source and destination projects exist (SELECT ... FOR UPDATE
   to lock the rows).
3. UPDATE all tasks from the source project to the destination.
4. COMMIT the transaction.

If anything fails, the transaction is rolled back automatically when the
`Transaction` object is dropped without calling `.commit()`.

### sqlx::Transaction

```rust
let mut tx = self.pool.begin().await?;

sqlx::query("UPDATE tasks SET project_id = $1 WHERE project_id = $2")
    .bind(to_id)
    .bind(from_id)
    .execute(&mut *tx)
    .await?;

tx.commit().await?;
```

Key points:
* `pool.begin()` starts a transaction and returns a `Transaction` handle.
* Pass `&mut *tx` (a mutable reference to the underlying connection) to queries.
* Call `.commit()` explicitly. If the `Transaction` is dropped without commit,
  it rolls back.

---

## Part 6: Complex Queries

### Full-text search

PostgreSQL full-text search converts text into a `tsvector` of lexemes and
matches it against a `tsquery`:

```sql
SELECT * FROM tasks
WHERE search_vector @@ plainto_tsquery('english', $1)
```

The `search_vector` column is a generated column so it stays in sync with
`title` and `description` automatically.

### JOIN queries

List tasks with their project name:

```sql
SELECT t.*, p.name AS project_name
FROM tasks t
JOIN projects p ON t.project_id = p.id
WHERE t.search_vector @@ plainto_tsquery('english', $1)
```

### Aggregation

Count tasks by status for a project:

```sql
SELECT status, COUNT(*) as count
FROM tasks
WHERE project_id = $1
GROUP BY status
```

### Indexes for performance

The migration creates indexes on columns used in WHERE and JOIN clauses:
* `idx_tasks_project_id` -- speeds up lookups by project.
* `idx_tasks_status`, `idx_tasks_priority` -- speeds up filtered queries.
* `idx_tasks_search` (GIN) -- required for efficient full-text search.

Without these indexes, PostgreSQL must scan every row (sequential scan). With
them, it can jump directly to matching rows (index scan).

---

## Part 7: Connection Pool

Opening a database connection is expensive (TCP handshake, TLS negotiation,
authentication). A **connection pool** maintains a set of open connections and
lends them out to concurrent requests.

### Configuring PgPool

```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(10)         // max concurrent connections
    .min_connections(2)          // keep at least 2 warm
    .acquire_timeout(Duration::from_secs(5))
    .idle_timeout(Duration::from_secs(600))
    .connect(&database_url)
    .await?;
```

### Health checks

sqlx periodically pings idle connections and replaces broken ones. You can also
run an explicit health check:

```rust
pool.acquire().await?.ping().await?;
```

### Integrating with axum

Pass the pool (or repositories built from it) as axum `State`:

```rust
let app = Router::new()
    .merge(project_routes().with_state(project_repo))
    .merge(task_routes().with_state(task_repo));
```

---

## Part 8: Integration with Axum

This is where everything comes together. The handlers in `src/handlers/` are
stubbed with `todo!()`. Implement them by:

1. Extracting the repository from `State`.
2. Validating the request body (using the `validator` crate).
3. Calling the appropriate repository method.
4. Mapping the result into an HTTP response (or an `AppError`).

Compare with web-1: instead of locking a `HashMap`, you call an async
repository method. The handler logic is almost identical, but the data now lives
in PostgreSQL and survives restarts.

---

## Building and running

```bash
# 1. Start PostgreSQL (Docker example)
docker run -d --name taskforge-pg \
  -e POSTGRES_PASSWORD=postgres \
  -p 5432:5432 \
  postgres:16

# 2. Create the database
createdb -h localhost -U postgres taskforge

# 3. Copy and edit .env
cp .env.example .env

# 4. Run the server (migrations run automatically)
cargo run --bin server

# 5. Test
curl http://localhost:3000/projects
```

## Running tests

The integration tests require a running PostgreSQL instance. Create a dedicated
test database to avoid polluting your development data:

```bash
createdb -h localhost -U postgres taskforge_test

DATABASE_URL=postgres://postgres:postgres@localhost/taskforge_test \
  cargo test
```

Tests use unique names (with embedded UUIDs) for isolation, so they can run
concurrently without conflicts.

---

## Exercises

1. Implement all `todo!()` methods in `src/repository/projects.rs`.
2. Implement all `todo!()` methods in `src/repository/tasks.rs`, paying special
   attention to `move_tasks_to_project` (must use a transaction).
3. Implement the handlers in `src/handlers/projects.rs` and
   `src/handlers/tasks.rs`.
4. Wire up the router in `src/bin/server.rs`.
5. Make all tests in `tests/db_tests.rs` pass.
6. (Stretch) Migrate from `query_as::<_, T>()` to `query_as!()` and run
   `cargo sqlx prepare` to generate offline query data.
7. (Stretch) Add a `RETURNING` clause to your UPDATE queries so you get the
   updated row back in a single round trip.
8. (Stretch) Add database-level pagination using cursors (keyset pagination)
   instead of OFFSET/LIMIT.
