use itertools::Itertools;
use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        x: [usize; q],
    }

    let mut ball: Vec<usize> = (1..=n).collect();
    let mut address: HashMap<usize, usize> =
        ball.iter().enumerate().map(|(i, v)| (*v, i)).collect();
    for x in x {
        let left = address[&x];
        let right = if address[&x] + 1 < n {
            address[&x] + 1
        } else {
            address[&x] - 1
        };

        address.entry(ball[left]).and_modify(|x| *x = right);
        address.entry(ball[right]).and_modify(|x| *x = left);

        ball.swap(left, right);
    }

    println!("{}", ball.iter().join(" "));
}
