use std::collections::VecDeque;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {(s, t): (Chars, Chars)}
    let mut s = VecDeque::from(s);
    let t = VecDeque::from(t);

    let mut ans = "No";
    for _ in 0..s.len() {
        s.rotate_right(1);
        if s == t {
            ans = "Yes";
            break;
        }
    }

    println!("{}", ans);
}
