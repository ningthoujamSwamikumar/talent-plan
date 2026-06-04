# Project 3: Iterator Forge

A lazy CSV query engine that teaches Rust's iterator system, closures, and lazy evaluation -- using only the standard library.

## Introduction: Why Iterators Matter

Rust's iterators are **zero-cost abstractions**. The compiler optimizes chains of iterator adaptors into tight loops equivalent to hand-written `for` code, but the source reads like a declarative pipeline. Three properties make them powerful:

1. **Zero-cost abstraction** -- no runtime overhead compared to manual loops.
2. **Lazy evaluation** -- no work is performed until a terminal operation consumes the iterator. This means you can build complex pipelines without allocating intermediate collections.
3. **Composability** -- small, focused adaptors (`map`, `filter`, `take`, `zip`, ...) snap together to express sophisticated transformations.

In this project you will build a lazy CSV query engine from scratch. Every query is an iterator pipeline: nothing happens until you ask for results.

---

## Part 1: CSV Row Iterator

**Goal:** Implement `CsvReader`, a struct that borrows a `&str` of CSV text and yields `Row` values lazily.

### What to implement

- `Row` (`src/row.rs`) -- a struct holding parallel `Vec<String>` vectors for headers and values. Implement:
  - `new(headers, values)` -- constructor.
  - `get(column)` -- look up a value by column name.
  - `headers()` / `values()` -- accessor slices.
  - `set(column, value)` -- mutate a single cell.
  - `select(columns)` -- produce a new `Row` with only the listed columns.
  - `Display` -- render as comma-separated values.

- `CsvReader` (`src/reader.rs`) -- stores a reference to the CSV text and tracks how far it has read. The first line is the header row; each subsequent line becomes a `Row`.

### Key concepts

```rust
impl<'a> Iterator for CsvReader<'a> {
    type Item = Row;            // The associated type tells Rust what this iterator yields.
    fn next(&mut self) -> Option<Self::Item> { /* ... */ }
}
```

`Iterator` has one required method (`next`) and one associated type (`Item`). Everything else -- `map`, `filter`, `take`, `collect`, and dozens more -- is provided automatically by the trait.

The lifetime `'a` ties `CsvReader` to the borrowed CSV text, ensuring the source data lives at least as long as the reader.

---

## Part 2: Filtering and Mapping

**Goal:** Add `.filter_rows(predicate)` and `.map_column(col, transform)` to `Query`.

### What to implement

- `filter_rows<F: FnMut(&Row) -> bool>` -- returns a new lazy `Query` that skips rows not matching the predicate.
- `map_column<F: FnMut(&str) -> String>` -- returns a new lazy `Query` that transforms a single column's value in every row.

### Key concepts: Closures

Closures in Rust are anonymous functions that can **capture variables** from their enclosing scope.

| Trait    | Meaning                          | Captures by          |
|----------|----------------------------------|----------------------|
| `FnOnce` | Can be called once               | move (takes ownership) |
| `FnMut`  | Can be called many times, may mutate state | `&mut` reference |
| `Fn`     | Can be called many times, read-only | `&` reference      |

Every closure implements `FnOnce`. If it does not move out of captured values it also implements `FnMut`. If it never mutates captured values it also implements `Fn`.

For iterator adaptors we typically require `FnMut` because the closure is called once per element (many times total) and may need to update a counter or accumulator.

```rust
let threshold = 30;                           // captured by the closure
query.filter_rows(|row| {
    let age: i32 = row.get("age").unwrap().parse().unwrap();
    age >= threshold                           // reads `threshold` via &
})
```

### Returning iterators from methods

The return type `Query<impl Iterator<Item = Row>>` uses `impl Trait` in return position to hide the concrete iterator type. The caller only knows it yields `Row` values -- the compiler monomorphises the chain behind the scenes.

---

## Part 3: Column Projection

**Goal:** Implement `.select(columns)` which keeps only the listed columns in each row.

### What to implement

- `select(&[&str])` on `Query` -- returns a lazy `Query` that maps each row through `Row::select`.

### Key concepts: Adaptor chaining

Iterator adaptors return new iterators. Because each adaptor is generic over its input, you can chain them freely:

```rust
Query::new(CsvReader::new(csv))
    .filter_rows(|r| r.get("active") == Some("true"))
    .map_column("name", |n| n.to_uppercase())
    .select(&["name", "email"])
    .collect_rows();
```

Under the hood this builds a nested type like `Select<MapColumn<Filter<CsvReader>>>`. The compiler inlines each `next()` call, producing a single loop with no heap allocations for intermediate results.

---

## Part 4: Aggregations

**Goal:** Implement consuming (terminal) methods on `Query`.

### What to implement

- `count(self) -> usize`
- `sum(self, column) -> f64`
- `avg(self, column) -> f64`
- `min(self, column) -> Option<f64>`
- `max(self, column) -> Option<f64>`
- `collect_rows(self) -> Vec<Row>`

These methods take `self` by value, consuming the iterator pipeline and producing a final result.

### Key concepts: `fold` and `collect`

`fold` is the most general consuming method -- every other consumer can be written in terms of it:

```rust
fn sum(self, column: &str) -> f64 {
    self.iter.fold(0.0, |acc, row| {
        acc + row.get(column)
                 .and_then(|v| v.parse::<f64>().ok())
                 .unwrap_or(0.0)
    })
}
```

`collect` is a specialised consumer that builds a collection. It delegates to the `FromIterator` trait on the target type (see Part 5).

---

## Part 5: Custom Collector

**Goal:** Implement `FromIterator<Row>` for `String` so that `CsvReader` results can be collected back into CSV text.

### What to implement

```rust
impl FromIterator<Row> for String {
    fn from_iter<T: IntoIterator<Item = Row>>(iter: T) -> Self { /* ... */ }
}
```

The resulting string should contain:
1. A header line (from the first row's headers).
2. One line per data row, with values comma-separated.

### Key concepts: `FromIterator` and `IntoIterator`

- `FromIterator<A>` -- "I know how to build myself from an iterator of `A`."
- `IntoIterator` -- "I can turn into an iterator." Any type implementing `IntoIterator` can be used in a `for` loop.

Together they power `collect()`:

```rust
let csv_string: String = CsvReader::new(data).collect();
```

The compiler sees that the target type is `String`, looks for `impl FromIterator<Row> for String`, and calls it.

---

## Part 6: Laziness Proof

**Goal:** Write tests that *prove* the pipeline is lazy.

### Strategy: side-effect counting

Use `std::cell::Cell<u32>` to count how many times a closure is invoked:

```rust
use std::cell::Cell;

let counter = Cell::new(0u32);
let pipeline = Query::new(CsvReader::new(csv))
    .filter_rows(|row| {
        counter.set(counter.get() + 1);
        true
    });

// Pipeline is built but nothing has executed yet.
assert_eq!(counter.get(), 0);
```

Only when you call a terminal method (`collect_rows`, `count`, ...) should the counter increase. This pattern proves that building the pipeline does zero work -- evaluation is entirely demand-driven.

---

## Building and testing

```bash
cargo test          # Run the full test suite
cargo clippy        # Lint
cargo doc --open    # View generated documentation
```

All tests live in `tests/tests.rs`. Start by making the Part 1 tests pass, then work through Parts 2-6 in order. Each part builds on the previous one.
