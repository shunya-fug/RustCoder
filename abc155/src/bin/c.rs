use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {n: usize, s: [String; n]}
    let mut counter: HashMap<String, usize> = HashMap::new();

    for s in s {
        let count = counter.entry(s).or_insert(0);
        *count += 1;
    }
    let count_max = counter.values().max().unwrap();

    let mut counter: Vec<String> = counter
        .iter()
        .filter(|(_, c)| *c == count_max)
        .map(|(s, _)| s)
        .cloned()
        .collect();
    counter.sort();

    for s in counter {
        println!("{}", s);
    }
}
