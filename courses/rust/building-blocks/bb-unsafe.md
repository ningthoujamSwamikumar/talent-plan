# Building Block: Unsafe Rust and FFI

Every large Rust codebase contains `unsafe`. Every production system interfaces
with C libraries, operating system APIs, or hardware. You cannot be a senior
Rust engineer and treat `unsafe` as "someone else's problem." This building block
teaches you to write sound unsafe code and to audit unsafe code others have
written.

## Readings

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) — the definitive guide
  to unsafe Rust. Read chapters 1-6 (through "Concurrency"). This is dense —
  take notes.
- [The Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/) — practical
  FFI patterns with C, Ruby, Python, etc. Focus on the C sections.
- [bindgen User Guide](https://rust-lang.github.io/rust-bindgen/) — generating
  Rust bindings from C headers automatically.
- [Miri: Undefined Behavior detector](https://github.com/rust-lang/miri) — run
  your unsafe code under Miri to detect UB. Essential for verification.
- [How to audit unsafe code (Aria Beingessner)](https://faultlore.com/blah/fix-rust-pointers/)
  — practical checklist for reviewing unsafe blocks.

## Key Concepts

**What `unsafe` unlocks (and only what it unlocks):**

1. Dereference raw pointers (`*const T`, `*mut T`)
2. Call `unsafe` functions (including `extern "C"` FFI)
3. Access mutable statics
4. Implement `unsafe` traits (`Send`, `Sync`)
5. Access fields of `union` types

**That's it.** Unsafe does NOT disable the borrow checker for references, does
NOT let you ignore lifetimes, and does NOT make undefined behavior "okay."

**Soundness:** An `unsafe` block is **sound** if no safe code calling into it
can trigger undefined behavior. The unsafe code must uphold all invariants that
safe code relies on. This is the central concept.

**Common UB patterns in Rust:**

| Pattern | Example | Detection |
|---------|---------|-----------|
| Use after free | Dereferencing a freed pointer | Miri, AddressSanitizer |
| Double free | Calling `drop` twice via raw pointer | Miri |
| Data race | Two threads writing without synchronization | ThreadSanitizer, Miri |
| Invalid value | Creating a `bool` that's not 0 or 1 | Miri |
| Aliasing violation | `&T` and `&mut T` existing simultaneously | Miri (stacked borrows) |
| Null dereference | Dereferencing null `*const T` | Miri, but often segfault |
| Uninitialized memory | Reading `MaybeUninit` before init | Miri |

**FFI patterns:**

```rust
// Calling a C function
extern "C" {
    fn strlen(s: *const libc::c_char) -> libc::size_t;
}

// Safe wrapper
fn safe_strlen(s: &CStr) -> usize {
    unsafe { strlen(s.as_ptr()) }
}

// The wrapper is sound because:
// 1. CStr guarantees null-termination (strlen's precondition)
// 2. CStr guarantees valid UTF-8 or at least valid C string
// 3. The pointer is valid for the duration of the call (borrow is alive)
```

**Ownership across FFI boundaries:**

The hardest part of FFI is memory ownership. When C allocates memory:
- Rust must NOT free it with Rust's allocator
- Rust must call the C library's free function
- Rust must track ownership to prevent use-after-free
- `Drop` implementations wrap the C free function

When Rust passes memory to C:
- Ensure the memory lives long enough (Pin or manual lifetime management)
- Ensure C doesn't store the pointer beyond the call (or document that it does)
- Use `Box::into_raw` / `Box::from_raw` for heap-allocated handoffs

## Exercises

1. **Write a safe wrapper for a C library.** Use `bindgen` to generate bindings
   for a simple C library (suggest: `libz` for compression, or write your own
   5-function C library). Wrap every function in a safe Rust API. Document the
   safety invariants of every `unsafe` block.

2. **Implement a simple allocator.** Write a bump allocator that implements
   `std::alloc::GlobalAlloc`. It should allocate from a fixed-size buffer. Test
   it with `#[global_allocator]`. This teaches you alignment, pointer arithmetic,
   and the allocator interface.

3. **Find the UB.** Run Miri on this code and explain each UB it detects:
   ```rust
   let mut v = vec![1, 2, 3];
   let ptr = v.as_mut_ptr();
   unsafe {
       v.push(4); // may reallocate, invalidating ptr
       *ptr = 10; // use after potential reallocation
   }
   ```

4. **Audit an unsafe block.** Pick any popular Rust crate (e.g., `bytes`,
   `crossbeam`, `dashmap`) and find an `unsafe` block. Read the surrounding
   code and safety comments. Write a 1-paragraph explanation of why it's sound
   (or why you think it might not be).

---

## Checklist

- [ ] You know exactly what `unsafe` unlocks (the 5 operations)
- [ ] You can explain soundness and give an example of an unsound safe API
- [ ] You can use `bindgen` to generate FFI bindings from C headers
- [ ] You can write safe wrappers that uphold invariants for `unsafe` blocks
- [ ] You can use Miri to detect undefined behavior in your code
- [ ] You understand memory ownership across FFI boundaries
- [ ] You can implement `Drop` for FFI resources to prevent leaks

Next: [Project: Unsafe Rust & FFI](../projects/senior-4/README.md)
