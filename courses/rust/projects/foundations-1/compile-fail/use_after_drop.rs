/// This program should NOT compile.
///
/// It tries to use a `&str` reference obtained from a `StringInterner` after
/// the interner has been dropped.  The borrow checker must reject this because
/// the reference borrows from the interner's internal storage, which is freed
/// when the interner goes out of scope.
use ownership_arena::StringInterner;

fn main() {
    let s;
    {
        let mut interner = StringInterner::new();
        let id = interner.intern("hello");
        s = interner.get(id).unwrap();
    } // `interner` is dropped here — `s` is now a dangling reference
    println!("{}", s); // ERROR: borrow of dropped value
}
