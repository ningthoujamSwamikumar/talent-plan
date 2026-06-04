# Project: Unsafe Rust & FFI

**Phase 6 — Senior Engineering Skills**

Every large Rust codebase contains `unsafe`. Every production system interfaces
with C libraries, OS APIs, or hardware. This project takes you from "I avoid
unsafe" to "I can write, review, and audit unsafe code confidently." You will
build safe wrappers around C libraries, implement a custom allocator, write
self-referential structures, and use Miri to verify soundness.

## Prerequisites

- Completion of all prior phases
- Building block bb-unsafe (Unsafe Rust and FFI)
- A C compiler (`gcc` or `clang`) for FFI parts
- Rust nightly (for Miri): `rustup toolchain install nightly`

## Deliverables

- [ ] Safe Rust wrappers for a C library with zero UB
- [ ] A bump allocator implementing `GlobalAlloc`
- [ ] A self-referential structure using `Pin`
- [ ] A concurrent data structure with `unsafe` internals
- [ ] All code passes `cargo +nightly miri test`
- [ ] Safety comments on every `unsafe` block

---

## Part 1: FFI Bindings with bindgen

Write safe Rust bindings for a C compression library.

**Setup:** Create a small C library (`compress.h` / `compress.c`) with these
functions:

```c
// Allocates a compression context. Caller must free with ctx_free.
compress_ctx* ctx_new(int level);

// Frees a compression context. Must not be called twice.
void ctx_free(compress_ctx* ctx);

// Compresses src into dst. Returns compressed size, or -1 on error.
// dst must have at least dst_capacity bytes.
int ctx_compress(compress_ctx* ctx, const char* src, int src_len,
                 char* dst, int dst_capacity);

// Decompresses src into dst. Returns decompressed size, or -1 on error.
int ctx_decompress(const char* src, int src_len,
                   char* dst, int dst_capacity);
```

**Tasks:**

1. Write a `build.rs` that compiles the C code using `cc::Build`:
   ```rust
   fn main() {
       cc::Build::new().file("c_src/compress.c").compile("compress");
   }
   ```

2. Use `bindgen` or write manual `extern "C"` declarations in `src/ffi.rs`:
   ```rust
   extern "C" {
       fn ctx_new(level: c_int) -> *mut CompressCtx;
       fn ctx_free(ctx: *mut CompressCtx);
       fn ctx_compress(ctx: *mut CompressCtx, src: *const c_char, src_len: c_int,
                       dst: *mut c_char, dst_capacity: c_int) -> c_int;
       fn ctx_decompress(src: *const c_char, src_len: c_int,
                         dst: *mut c_char, dst_capacity: c_int) -> c_int;
   }
   ```

3. Write a safe wrapper in `src/compress.rs`:
   ```rust
   pub struct Compressor {
       ctx: NonNull<CompressCtx>,
   }

   impl Compressor {
       pub fn new(level: u32) -> Result<Self, CompressError> { todo!() }
       pub fn compress(&mut self, input: &[u8]) -> Result<Vec<u8>, CompressError> { todo!() }
   }

   impl Drop for Compressor {
       fn drop(&mut self) { todo!() }
   }

   // SAFETY: CompressCtx is only accessed through &mut self, so no aliasing.
   // The C library documents that ctx_compress is not thread-safe per-context
   // but separate contexts can be used from different threads.
   unsafe impl Send for Compressor {}
   ```

**Safety invariants to document:**
- `ctx_new` returns null on failure → check and return `Err`
- `ctx_free` must be called exactly once → `Drop` handles this
- `ctx_compress` requires valid pointers → pass from `&[u8]` slices
- Buffer must have sufficient capacity → allocate before calling
- Double-free prevention → `Drop` sets internal state (or use `Option`)

**Tests:**
- `roundtrip_compress_decompress` — data survives compress → decompress
- `compress_error_on_invalid_level` — returns error, not panic
- `compress_handles_empty_input` — edge case
- `compress_handles_large_input` — 1MB of data
- All tests pass under `cargo +nightly miri test` (if pure-Rust parts allow)

## Part 2: Custom Bump Allocator

Implement a bump allocator that allocates from a fixed-size buffer.

**Requirements:**
```rust
pub struct BumpAllocator {
    heap: UnsafeCell<[u8; HEAP_SIZE]>,
    next: AtomicUsize,
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { todo!() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocators don't deallocate individual allocations
    }
}
```

**Implementation details:**
1. Maintain a `next` pointer into the buffer
2. On `alloc`: align `next` to `layout.align()`, check that `next + layout.size()` 
   doesn't exceed the buffer, advance `next`, return the pointer
3. On `dealloc`: no-op (bump allocators free everything at once)
4. Use `AtomicUsize` and `compare_exchange` for thread safety
5. Implement a `reset()` method that resets `next` to 0

**Alignment:**
```rust
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
```

**Tests:**
- `bump_allocator_basic` — allocate and use a `Vec<u32>`
- `bump_allocator_alignment` — allocated pointers are properly aligned
- `bump_allocator_overflow` — returns null when buffer is exhausted
- `bump_allocator_concurrent` — works from multiple threads
- Passes `cargo +nightly miri test`

## Part 3: Pin and Self-Referential Structures

Build a structure that contains a reference to its own data — the problem that
`Pin` was designed to solve.

**Scenario:** An `InternedString` pool that stores strings and hands out
references into its own buffer. The pool must not be moved after references
are created.

**Implementation:**
```rust
pub struct StringPool {
    buffer: String,
    entries: Vec<(*const str, usize)>, // (pointer into buffer, length)
    _pin: PhantomPinned,
}

impl StringPool {
    pub fn new() -> Pin<Box<Self>> {
        Box::pin(Self {
            buffer: String::new(),
            entries: Vec::new(),
            _pin: PhantomPinned,
        })
    }

    pub fn intern(self: Pin<&mut Self>, s: &str) -> usize {
        // SAFETY: We don't move `self` or invalidate existing pointers.
        // We only append to `buffer` (which may reallocate)...
        // Wait — that invalidates pointers! Fix this.
        todo!()
    }

    pub fn get(self: Pin<&Self>, index: usize) -> &str {
        todo!()
    }
}
```

**The learning:** You'll discover that `String` reallocation invalidates
pointers. Solutions:
1. Use indices instead of pointers (the correct approach for most cases)
2. Use a stable-address buffer (`Vec<Box<str>>`)
3. Pre-allocate with known capacity

**Tests:**
- `string_pool_intern_and_get` — basic round-trip
- `string_pool_multiple_entries` — 100 interned strings
- `string_pool_is_not_unpin` — static assertion that `StringPool: !Unpin`
- `string_pool_zero_copy` — getting the same string twice returns the same data

## Part 4: Lock-Free Concurrent Stack

Implement a lock-free stack using `AtomicPtr` and compare-and-swap.

```rust
pub struct ConcurrentStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> ConcurrentStack<T> {
    pub fn new() -> Self { todo!() }
    pub fn push(&self, val: T) { todo!() }
    pub fn pop(&self) -> Option<T> { todo!() }
}

unsafe impl<T: Send> Send for ConcurrentStack<T> {}
unsafe impl<T: Send> Sync for ConcurrentStack<T> {}
```

**Implementation:**
- `push`: allocate node, CAS head to new node
- `pop`: CAS head to head.next, return old head's data
- Use `Ordering::AcqRel` for CAS, `Ordering::Acquire` for loads

**The ABA problem:** Explain in comments why naive CAS is vulnerable to ABA
(thread 1 reads head=A, thread 2 pops A and B then pushes A back, thread 1
CAS succeeds but the list is corrupted). Describe mitigation strategies:
tagged pointers, hazard pointers, epoch-based reclamation (crossbeam).

**Tests:**
- `stack_push_pop_single_thread` — basic LIFO behavior
- `stack_concurrent_push_pop` — 10 threads, 1000 pushes each, all items
  accounted for
- `stack_is_send_sync` — static assertions
- `stack_empty_pop_returns_none`

**Note:** A fully correct lock-free stack requires epoch-based memory
reclamation (to avoid use-after-free on `pop`). Document this limitation
and reference `crossbeam-epoch` as the production solution.

## Part 5: Unsafe Audit

Review and fix the unsafe code in `src/audit.rs`. This file contains 5
`unsafe` blocks, each with a subtle bug:

1. Use-after-free via `Vec` reallocation
2. Aliasing violation (`&T` and `&mut T` coexisting)
3. Data race on `static mut`
4. Invalid `transmute` between incompatible types
5. Uninitialized memory read via `MaybeUninit`

For each:
- Explain the bug in a comment
- Fix the code
- Add a `// SAFETY:` comment explaining why the fix is sound
- Verify with `cargo +nightly miri test`

## Testing

```
cargo test
cargo +nightly miri test  # verify no undefined behavior
```

## What You Will Learn

- How to write FFI bindings for C libraries with safe Rust wrappers
- Build scripts (`build.rs`) for compiling C code
- How `GlobalAlloc` works and how to implement a custom allocator
- Why `Pin` exists and how to build self-referential structures
- Lock-free programming with atomics and CAS
- How to audit unsafe code for soundness
- Using Miri to detect undefined behavior
- Memory ordering: Acquire, Release, AcqRel, SeqCst
