/// Exercise 1: Implement Iterator for a range type
fn main() {
    struct CountDown(u32);

    impl Iterator for CountDown {
        type Item = u32;
        fn next(&mut self) -> Option<u32> {
            // Return current value, decrement, stop at 0
            let current = self.0;

            if current > 0 {
                self.0 -= 1;
                Some(current)
            } else {
                None
            }
        }
    }

    // Should work:
    let v: Vec<u32> = CountDown(5).collect();
    assert_eq!(v, vec![5, 4, 3, 2, 1]);
    println!("Assertion Passed!");
}
