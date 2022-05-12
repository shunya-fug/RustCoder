use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {(a, b, k): (isize, isize, isize)}
    let mut register: HashSet<isize> = HashSet::new();

    for n in a..(a + k).min(b) {
        if !register.contains(&n) {
            register.insert(n);
            println!("{}", n);
        }
    }
    for n in (b - k + 1).max(a)..=b {
        if !register.contains(&n) {
            register.insert(n);
            println!("{}", n);
        }
    }
}
