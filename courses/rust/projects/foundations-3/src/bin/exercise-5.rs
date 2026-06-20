/// Exercise 5: Prove laziness
fn main() {
    // Write an iterator adaptor that logs when each element is produced.
    // Use it to verify that .map().filter().take(2) only processes elements until 2 pass the filter, not all elements.

    let nums = vec![5; 32];
    let mut count = 10;
    let result: Vec<i32> = nums
        .iter()
        .map(|n| {
            println!("{n}");
            *n
        })
        .filter(|_| {
            count -= 1;
            count > 0
        })
        .take(2)
        .collect();

    println!("{:#?}", result);
    assert_eq!(result.len(), 2);
    println!("Assertion Passed.");
}
