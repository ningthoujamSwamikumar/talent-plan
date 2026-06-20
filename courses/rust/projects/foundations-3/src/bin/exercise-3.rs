/// Exercise 3: Iterator chains
fn main() {
    // Rewrite this imperative code as an iterator chain:

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut result = Vec::new();
    for n in &numbers {
        if n % 2 == 0 {
            let doubled = n * 2;
            if doubled > 10 {
                result.push(doubled);
            }
        }
    }
    // result should be [12, 16, 20]

    let result2: Vec<i32> = numbers
        .iter()
        .filter(|&n| n % 2 == 0)
        .map(|&n| n * 2)
        .filter(|&n| n > 10)
        .collect();
    assert_eq!(result2, vec![12, 16, 20]);
    println!("Assertion Passed.");
}
