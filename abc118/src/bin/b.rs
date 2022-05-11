use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
    }
    let mut food: HashSet<usize> = (1..=m).collect();

    for _ in 0..n {
        input! {k: usize, a: [usize; k]}

        food = food
            .intersection(&a.into_iter().collect::<HashSet<usize>>())
            .cloned()
            .collect();
    }

    println!("{}", food.len());
}
