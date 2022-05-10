use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let order = a
        .iter()
        .enumerate()
        .map(|(i, a)| (*a, i + 1))
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .map(|(_, i)| i)
        .join(" ");

    println!("{}", order);
}
