use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        x: [usize; q],
    }

    let mut ball: Vec<usize> = (1..=n).collect();
    let mut address: Vec<usize> = (1..=n).collect();
    address.insert(0, 0);
    for x in x {
        let i = address[x];
        let j = if address[x] + 1 < ball.len() {
            address[x] + 1
        } else {
            0
        };
        ball.swap(i, j);
        address.swap(ball[i], ball[j]);
    }

    println!("{}", ball.iter().join(" "));
}
