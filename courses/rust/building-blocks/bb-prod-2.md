# Building Block Prod-2: Advanced Testing Strategies

**Prerequisites**: [Project: Observability Stack](../projects/prod-1/README.md).

Before starting [Project: Testing Mastery](../projects/prod-2/README.md), complete
the readings and exercises below.

## What to read

- [Proptest documentation](https://docs.rs/proptest/latest/proptest/).
  Property-based testing for Rust. Instead of testing specific examples, you define
  properties that should hold for ALL inputs.

- [cargo-fuzz book](https://rust-fuzz.github.io/book/).
  Fuzz testing feeds random/mutated inputs to your code to find crashes and panics.

- [Insta documentation](https://docs.rs/insta/latest/insta/).
  Snapshot testing — capture output and automatically detect regressions.

- [Mockall documentation](https://docs.rs/mockall/latest/mockall/).
  Creating mock implementations of traits for unit testing.

- [Rust Testing Patterns (blog)](https://blog.logrocket.com/rust-testing-strategies/).
  Overview of testing strategies in Rust: unit, integration, property-based, doc tests.

## Key concepts

### Property-Based Testing

Instead of `assert_eq!(sort(vec![3,1,2]), vec![1,2,3])`, test the *property*:

```rust
proptest! {
    #[test]
    fn sort_preserves_length(ref v in prop::collection::vec(any::<i32>(), 0..100)) {
        let sorted = sort(v.clone());
        assert_eq!(sorted.len(), v.len());
    }

    #[test]
    fn sort_is_ordered(ref v in prop::collection::vec(any::<i32>(), 0..100)) {
        let sorted = sort(v.clone());
        for w in sorted.windows(2) {
            assert!(w[0] <= w[1]);
        }
    }
}
```

### Test Organization

```
tests/
├── unit/          — test individual functions in isolation
├── integration/   — test API endpoints with real database
├── property/      — proptest-based property tests
└── snapshot/      — insta snapshot tests for API responses
```

### The Testing Pyramid

```
      ╱╲
     ╱  ╲      E2E tests (few, slow, high confidence)
    ╱────╲
   ╱      ╲    Integration tests (moderate count)
  ╱────────╲
 ╱          ╲   Unit tests (many, fast, focused)
╱────────────╲
```

## Exercises

**Exercise 1**: Write 3 proptest properties for a `HashMap`-based cache:
(a) get after set returns the value, (b) set overwrites previous values,
(c) remove makes get return None.

**Exercise 2**: Use `insta` to snapshot-test a JSON API response. Change the
response format and see how insta detects the regression.

**Exercise 3**: Create a trait and mock it with `mockall`. Test that a function
calls the mock's methods in the expected order.

## You're ready when...

- [ ] You can write proptest properties that test invariants
- [ ] You understand snapshot testing with insta
- [ ] You can mock traits for unit testing
- [ ] You know when to use each testing strategy

Next: [Project: Testing Mastery](../projects/prod-2/README.md)
