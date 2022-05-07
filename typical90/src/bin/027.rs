use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: usize}

    let mut register = HashSet::new();
    for i in 1..=n {
        input! {s: String}
        if !register.contains(&s) {
            register.insert(s);
            println!("{}", i);
        }
    }
}
