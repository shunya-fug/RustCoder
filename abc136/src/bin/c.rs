use std::cmp::Ordering;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [isize; n]
    }

    let mut ans = "Yes";
    for i in (0..n).rev().skip(1) {
        match h[i].cmp(&(h[i + 1] + 1)) {
            Ordering::Equal => h[i] -= 1,
            Ordering::Greater => {
                ans = "No";
                break;
            }
            _ => continue,
        }
    }

    println!("{}", ans);
}
