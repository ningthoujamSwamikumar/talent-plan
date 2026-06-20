/// Exercise 2: Closure capture modes
///
fn main() {
    // Predict the output or compiler error for each:

    // A
    let name = String::from("Alice");
    let greet = || println!("Hello, {}!", name);
    greet();
    greet();
    println!("{}", name); // Does this work?
    // Yes the above will work, since we closure captured immutable reference

    // B
    let mut count = 0;
    let mut increment = || {
        count += 1;
        count
    };
    println!("{}", increment()); // will work
    println!("{}", increment()); // this seems like this will not work because count moved in the first call of increment closure
    // but the above will work because, i32 implements copy trait, which allows it to copy and return the copy valued instead of the original

    // C
    let name = String::from("Alice");
    let consume = || {
        drop(name);
    };
    consume(); // will work
    // consume(); // Does this work? => won't work, as name has already been dropped in the first call
    // uncomment the above out and try to run it. and then comment and run again. 
}
