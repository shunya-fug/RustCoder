use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {(n, a, b): (usize, usize, usize)}

    let ans = match (b - a) % 2 {
        0 => (b - a) / 2,
        1 => (a - 1).min(n - b) + 1 + (b - a - 1) / 2,
        _ => panic!(),
    };

    println!("{}", ans);
}
