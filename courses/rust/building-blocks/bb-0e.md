# Building Block: Procedural Macros

Procedural macros are how Rust libraries like `serde`, `tokio`, and `thiserror`
eliminate boilerplate. Understanding them is required for contributing to the
Rust ecosystem and for building production libraries. This is not optional
knowledge for a senior engineer — you will encounter proc macros in every
large codebase.

## Readings

- [The Rust Reference: Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)
  — official specification. Read end-to-end.
- [proc-macro-workshop](https://github.com/dtolnay/proc-macro-workshop) — David
  Tolnay's guided exercises. You will complete several of these.
- [syn crate documentation](https://docs.rs/syn/latest/syn/) — the parsing
  library. Focus on `DeriveInput`, `Fields`, `Data`, and `parse_macro_input!`.
- [quote crate documentation](https://docs.rs/quote/latest/quote/) — the code
  generation library. Focus on `quote!`, `#var` interpolation, and `#(#iter)*`
  repetition.
- [How derive macros work under the hood](https://blog.turbo.fish/proc-macro-basics/)
  — practical walkthrough with examples.

## Key Concepts

**Three types of procedural macros:**

1. **Derive macros** (`#[derive(MyMacro)]`) — generate impl blocks for a struct
   or enum. Most common type. Used by serde, thiserror, clap, sqlx, etc.

2. **Attribute macros** (`#[my_attribute]`) — transform the item they're attached
   to. Used by tokio (`#[tokio::main]`), axum (`#[debug_handler]`), async-trait.

3. **Function-like macros** (`my_macro!(...)`) — called like functions. Used by
   sqlx (`query!`), lazy_static, etc.

**The proc macro pipeline:**

```
Source code (tokens) → Parse with syn → Transform → Generate with quote → Output tokens
```

Every proc macro follows this pattern. The craft is in the middle: deciding what
to parse, how to transform it, and what to generate.

**Why this matters for senior work:**

- Reading `serde`'s derive implementation teaches you more about Rust's type
  system than any tutorial
- Contributing to libraries often requires modifying proc macros
- Building internal tools (custom test frameworks, configuration DSLs, code
  generators) uses proc macros
- Debugging macro-generated code requires understanding expansion (`cargo expand`)

## Exercises

1. Set up a proc macro crate. Create `my-macros/` with `Cargo.toml` containing
   `[lib] proc-macro = true`. Write a derive macro called `HelloMacro` that
   generates `impl HelloMacro for T { fn hello() -> String { "Hello from T" } }`.
   Verify it works on a struct.

2. Implement a `Builder` derive macro following steps 1-7 of the
   [proc-macro-workshop builder exercise](https://github.com/dtolnay/proc-macro-workshop/tree/master/builder).
   This teaches you: parsing struct fields with `syn`, generating methods with
   `quote`, handling `Option` fields, and producing compile errors from macros.

3. Use `cargo expand` to see what `#[derive(Debug)]` and `#[derive(Clone)]`
   generate for a struct with 3 fields. Read the output carefully — this is what
   your macros will produce.

4. Read the source code of `thiserror`'s derive macro (about 500 lines). Note
   how it handles `#[error("...")]` attributes, `#[from]` for automatic
   conversion, and `#[source]` for error chaining. You don't need to understand
   every line — understand the pattern.

---

## Checklist

- [ ] You can create a proc macro crate and export a derive macro
- [ ] You can parse struct fields using `syn::DeriveInput`
- [ ] You can generate code using `quote!` with interpolation
- [ ] You can produce compile errors from macros using `syn::Error`
- [ ] You understand the difference between derive, attribute, and function-like macros
- [ ] You can use `cargo expand` to inspect macro output

Next: [Project: Procedural Macro Workshop](../projects/foundations-5/README.md)
