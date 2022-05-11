use proconio::{fastout, input, marker::Chars};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {w: Chars}

    let mut counter: HashMap<char, usize> = HashMap::new();
    for c in w {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    if counter.values().all(|n| n % 2 == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
