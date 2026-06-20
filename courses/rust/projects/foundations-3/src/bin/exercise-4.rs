use std::collections::HashMap;

/// Exercise 4: Implement FromIterator
fn main() {
    // Write a Histogram type that can be collected from an iterator of strings, counting the occurrences of each:

    struct Histogram<'a> {
        freq: HashMap<&'a str, i32>,
    }

    impl<'a> FromIterator<&'a str> for Histogram<'a> {
        fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
            let mut freq = HashMap::new();
            for k in iter {
                if let Some(v) = freq.get_mut(k) {
                    *v += 1;
                } else {
                    freq.insert(k, 1);
                };
            }

            Self { freq }
        }
    }

    impl Histogram<'_> {
        fn count(&self, key: &str) -> i32 {
            if let Some(&f) = self.freq.get(key) {
                f
            } else {
                0
            }
        }
    }

    let words = vec!["apple", "banana", "apple", "cherry", "banana", "apple"];
    let hist: Histogram = words.into_iter().collect();
    assert_eq!(hist.count("apple"), 3);
    println!("Assertion Passed.");
    // println!("{:#?}", words); // this will give compiler error, because into_iter or from_iter consumes the words
}
