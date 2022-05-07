use num::pow;
use proconio::input;

fn main() {
    input! {
        a: u128,
        b: usize,
        c: u128,
    }

    let ans = if a < pow(c, b) { "Yes" } else { "No" };
    println!("{}", ans);
}
