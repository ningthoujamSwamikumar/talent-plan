# Testing Mastery

Phase 3, Project 2 of the Practical Networked Applications in Rust course.

## Introduction

In this project, **you are the test suite**. You will not write library code -- instead,
you will write comprehensive tests for a provided task management library. The library
is fully implemented but contains subtle bugs hidden in edge cases. Your job is to find
them through disciplined testing.

This project teaches you five pillars of Rust testing:

1. Unit tests that verify individual functions
2. Property-based tests that verify invariants across random inputs
3. Snapshot tests that lock down serialization formats
4. Mock-based tests that isolate components
5. Integration tests that verify the full stack

The library under test is an in-memory task store. It supports creating, reading,
updating, deleting, and listing tasks. Tasks have a status (Todo, InProgress, Done),
a priority (Low, Medium, High, Critical), and standard metadata like title, description,
and timestamps.

## Part 1: Unit Tests

**File:** `tests/unit_tests.rs`

Write tests for individual functions in the library:

- **Validation**: Verify that creating a task with an empty title returns an error.
  Verify that invalid priority values are rejected.
- **Status transitions**: Verify that valid transitions (Todo -> InProgress,
  InProgress -> Done) succeed, and that invalid transitions are rejected.
- **Priority ordering**: Verify that priorities compare correctly
  (Critical > High > Medium > Low).
- **CRUD basics**: Verify that creating a task returns a valid ID, that getting
  a task by ID returns the correct task, and that updating fields persists correctly.

Tips:
- Each test should verify exactly one behavior.
- Use descriptive test names that read like specifications.
- Remember: the library has bugs. Some of your tests should *fail* against the
  provided implementation. That is the point.

## Part 2: Property-Based Tests with Proptest

**File:** `tests/property_tests.rs`

Property-based tests verify invariants that should hold for *any* input, not just
the specific cases you thought of. Use the `proptest` crate.

Properties to test:

- **Store consistency**: For any sequence of create/delete operations, the store's
  count should equal the number of creates minus the number of successful deletes.
- **ID uniqueness**: For any number of created tasks, all IDs should be distinct.
- **Round-trip serialization**: For any valid task, serializing to JSON and
  deserializing back should produce an equal task.
- **Idempotent get**: Getting the same task twice should return identical results.
- **List completeness**: After creating N tasks, listing all tasks should return
  exactly N tasks.
- **Valid status transitions only**: Applying a random valid transition should
  succeed; applying a random invalid transition should fail.
- **Title validation**: Any non-empty string should be accepted as a title;
  any empty string should be rejected.
- **Priority ordering is total**: For any two priorities, exactly one of
  less-than, equal, or greater-than holds.

## Part 3: Snapshot Tests with Insta

**File:** `tests/snapshot_tests.rs`

Snapshot tests lock down the exact output of serialization, error messages, and
display formats. Use the `insta` crate.

Snapshots to capture:

- JSON serialization of a task with all fields populated
- JSON serialization of a minimal task (only required fields)
- Display format of each error variant
- JSON serialization of a list of tasks with mixed statuses
- Debug output of the TaskStatus enum variants
- JSON serialization of a task after a status transition

Run `cargo insta review` after your first run to accept the initial snapshots.
On subsequent runs, any change in output will cause a test failure -- which is
exactly what you want for catching unintended format changes.

## Part 4: Mock Tests with Mockall

**File:** `tests/mock_tests.rs`

The library defines a `Repository` trait. Use the `mockall` crate to create a
mock implementation, then test service-level logic in isolation.

Mock scenarios:

- **Create succeeds**: Mock the repository to return Ok, verify the service
  calls create exactly once.
- **Create fails**: Mock the repository to return an error, verify the service
  propagates the error.
- **Get returns task**: Mock the repository to return a task, verify the service
  returns it unmodified.
- **Get returns not found**: Mock the repository to return None, verify the
  service returns a NotFound error.
- **Delete calls repository**: Verify that deleting a task calls the repository's
  delete method with the correct ID.
- **List with filter**: Mock the repository to return a set of tasks, verify the
  service correctly filters by status.

## Part 5: Test Fixtures and Factories

Both the unit test and integration test files use factory functions to generate
test data. Use the `fake` crate to generate realistic random data.

Create helper functions:

- `make_task()` -- returns a task with random but valid fields
- `make_create_request()` -- returns a valid CreateTask with random data
- `make_store_with_tasks(n)` -- returns a store pre-populated with n random tasks

These helpers should live in a shared test utilities module or be duplicated
in each test file as needed.

## Part 6: Integration Tests

**File:** `tests/integration_tests.rs`

Integration tests exercise the full stack: the service layer backed by the real
in-memory store. No mocks.

Scenarios:

- **Full lifecycle**: Create a task, update it, transition its status through
  all valid states, then delete it.
- **Pagination**: Create 25 tasks, paginate through them 10 at a time, verify
  all tasks are returned exactly once.
- **Concurrent-style operations**: Create tasks, delete some, update others,
  then verify the final state is consistent.
- **Error accumulation**: Perform multiple invalid operations and verify each
  returns the correct error type.
- **Search and filter**: Create tasks with different statuses and priorities,
  filter by each, verify results.
- **Bulk operations**: Create 100 tasks, delete all of them, verify the store
  is empty.

## Building and Running

```bash
# Build the library (should compile cleanly)
cargo build

# Run all tests (some will fail -- that's expected once you write them)
cargo test

# Run only unit tests
cargo test --test unit_tests

# Run only property tests
cargo test --test property_tests

# Run snapshot tests and review
cargo test --test snapshot_tests
cargo insta review

# Run mock tests
cargo test --test mock_tests

# Run integration tests
cargo test --test integration_tests
```

## Grading

Your tests are graded on:

1. **Coverage**: Do your tests exercise all public functions and edge cases?
2. **Bug detection**: Do your tests catch the intentional bugs in the library?
3. **Property quality**: Are your proptest properties meaningful invariants,
   not just re-implementations of the code under test?
4. **Isolation**: Do your mock tests truly isolate the service from the repository?
5. **Readability**: Are your tests well-named and self-documenting?

## Known Bugs

The library contains intentional bugs. Here are hints about where to look:

1. Try paginating through a list of tasks. Do you get every task exactly once?
2. Try transitioning a completed task back to Todo. Should that work?
3. Try deleting a task that does not exist. What should happen?
4. Think about what happens if two tasks are created at the exact same nanosecond.

Your tests should detect all four of these bugs.
