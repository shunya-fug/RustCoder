use proconio::{fastout, input, marker::Chars};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {(a, b): (Chars, Chars)}

    let mut ans = "EQUAL";
    match a.len().cmp(&b.len()) {
        Ordering::Greater => ans = "GREATER",
        Ordering::Less => ans = "LESS",
        Ordering::Equal => {
            for (a, b) in a.iter().zip(b) {
                if a > &b {
                    ans = "GREATER";
                    break;
                }
                if a < &b {
                    ans = "LESS";
                    break;
                }
            }
        }
    }

    println!("{}", ans);
}
