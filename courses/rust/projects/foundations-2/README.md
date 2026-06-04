# Project 2: Type Machinist

## Introduction

In this project you will build a **composable data transformation pipeline** in
Rust. The pipeline accepts a string, passes it through a chain of transforms,
and produces a new string (or an error). Along the way you will master some of
the most important concepts in the Rust type system:

* **Traits** -- the core abstraction mechanism for shared behavior.
* **Associated types** -- how to give each trait implementation its own output
  and error types without making the trait generic.
* **Generics and trait objects** -- when to use static dispatch (`impl Trait`,
  generics) versus dynamic dispatch (`dyn Trait`).
* **Error handling with `thiserror`** -- deriving `Display` and `Error`
  implementations, wrapping lower-level errors, and the `From` conversion
  pattern that powers the `?` operator.
* **Standard conversion traits** -- `FromStr`, `TryFrom`, `From`, and when to
  reach for each one.
* **The builder pattern** -- a fluent API for constructing complex objects step
  by step.

By the end of the project every `todo!()` in the starter code will be replaced
with working code, and the full test suite will pass.

---

## Part 1: The Transform Trait

Open `src/transforms.rs`. You will find a trait definition:

```rust
pub trait Transform {
    type Error;
    fn transform(&self, input: &str) -> Result<String, Self::Error>;
    fn name(&self) -> &str;
}
```

### Why an associated type instead of a generic parameter?

A generic parameter (`trait Transform<E>`) would allow a single struct to
implement the trait multiple times -- once per error type. That is rarely what
we want for a transform. Each transform has exactly **one** natural error type,
so we model it as an associated type. The compiler can then infer the error type
from the implementing struct without the caller having to specify it.

### Your task

Understand the trait. You do not need to change the definition -- you will
implement it for several structs in the next part.

---

## Part 2: Built-in Transforms

In the same file you will find five empty structs:

| Struct            | Behavior                                                                 |
|-------------------|--------------------------------------------------------------------------|
| `Uppercase`       | Convert the input to uppercase (`str::to_uppercase`).                    |
| `Lowercase`       | Convert the input to lowercase (`str::to_lowercase`).                    |
| `TrimWhitespace`  | Trim leading and trailing whitespace (`str::trim`), return the result.   |
| `CsvToJson`       | Parse the input as CSV (first line = headers, remaining lines = rows) and produce a JSON array of objects. |
| `JsonPrettify`    | Parse the input as JSON and re-serialize it with pretty printing (`serde_json::to_string_pretty`). |

Each struct should have its **own error type** (or use an existing one where
appropriate). For `Uppercase`, `Lowercase`, and `TrimWhitespace` the operation
is infallible on valid `&str`, so you may use `std::convert::Infallible` as the
error type -- or define a trivial custom error. `CsvToJson` can fail if the
input is malformed, and `JsonPrettify` can fail if the JSON is invalid.

### Your task

1. Implement `Transform` for each of the five structs.
2. Define any error types you need (e.g., `CsvParseError`, `JsonError`).
3. Make sure each `name()` method returns a descriptive string
   (e.g., `"uppercase"`, `"csv_to_json"`).

### CsvToJson specification

* The first line of the input contains comma-separated header names.
* Each subsequent non-empty line contains comma-separated values.
* The output is a JSON array where each element is an object mapping headers to
  values.
* Example:

  Input:
  ```
  name,age
  Alice,30
  Bob,25
  ```

  Output:
  ```json
  [{"name":"Alice","age":"30"},{"name":"Bob","age":"25"}]
  ```

* If a row has a different number of fields than the header, return an error.

---

## Part 3: Error Unification

Open `src/error.rs`. You will find a stub for `PipelineError`:

```rust
#[derive(Debug, Error)]
pub enum PipelineError {
    // TODO: add variants for each transform error type
}
```

When transforms are composed into a pipeline, we need a **single error type**
that can represent any transform failure. The standard Rust approach is:

1. Define an enum with one variant per source error.
2. Derive `thiserror::Error` so that `Display` and `std::error::Error` are
   implemented automatically.
3. Implement `From<SourceError> for PipelineError` for each source error so
   that the `?` operator works seamlessly.

`thiserror` makes step 3 trivial with its `#[from]` attribute.

### Your task

1. Add variants to `PipelineError` for every error type your transforms can
   produce (at minimum: CSV parse errors, JSON errors, and a catch-all or
   custom variant).
2. Use `#[error("...")]` to give each variant a human-readable message.
3. Use `#[from]` where appropriate to auto-generate `From` implementations.

---

## Part 4: The Pipeline

Open `src/pipeline.rs`. You will find a `Pipeline` struct that holds a
`Vec<Box<dyn Transform<Error = PipelineError>>>`.

This uses **trait objects** for dynamic dispatch. We erase the concrete type of
each transform so that a single `Vec` can hold different transform types. The
bound `Error = PipelineError` ensures every boxed transform produces the same
error type, which is essential for chaining.

### Object safety

A trait is object-safe if all its methods:
* Do not return `Self`.
* Do not use generic type parameters.
* Have a receiver (`&self`, `&mut self`, etc.).

Our `Transform` trait satisfies all three conditions, so it can be used as a
trait object.

### Your task

1. Implement a `new()` constructor for `Pipeline`.
2. Implement an `add_transform` method that pushes a boxed transform.
3. Implement a `run(&self, input: &str) -> Result<String, PipelineError>`
   method that feeds the input through each transform in order, passing the
   output of one as the input to the next.

---

## Part 5: Custom Conversions

Still in `src/transforms.rs` (or a new module if you prefer), define:

### `TransformKind` enum

```rust
pub enum TransformKind {
    Uppercase,
    Lowercase,
    TrimWhitespace,
    CsvToJson,
    JsonPrettify,
}
```

Implement `FromStr` for `TransformKind` so that `"uppercase".parse::<TransformKind>()`
returns `Ok(TransformKind::Uppercase)`, and similarly for the other variants.
Return a `PipelineError` (or a dedicated error type) for unrecognized strings.

### `TryFrom<&str>` for structured data

Implement `TryFrom<&str>` for a `TransformSpec` struct that parses a string
like `"uppercase"` or `"csv_to_json"` into a spec containing the
`TransformKind`. This demonstrates the standard conversion trait hierarchy:
`FromStr` is for owned-parse, `TryFrom<&str>` is for borrowed-parse.

### Your task

1. Define `TransformKind` and implement `FromStr`.
2. Define `TransformSpec` and implement `TryFrom<&str>`.

---

## Part 6: Builder Pattern

In `src/pipeline.rs` you will find a `PipelineBuilder` stub.

The builder pattern lets callers construct a `Pipeline` step by step with a
fluent API:

```rust
let pipeline = PipelineBuilder::new()
    .add(Uppercase)
    .add(TrimWhitespace)
    .build();
```

### Your task

1. Implement `PipelineBuilder::new()`.
2. Implement `add<T>(mut self, transform: T) -> Self` where `T`
   implements the `Transform` trait with `Error = PipelineError` and is
   `'static`. Note: this takes `self` by value (with `mut`) and returns `Self`,
   enabling owned method chaining.
3. Implement `build(self) -> Pipeline`.

---

## Running the tests

```bash
cargo test
```

All tests live in `tests/tests.rs`. They exercise every part of the project.
Your implementation is complete when every test passes.

Good luck, and have fun building your Type Machinist!
