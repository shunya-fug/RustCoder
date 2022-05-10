use proconio::marker::Chars;
use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {s: Chars}
    let len = s.len();
    let set: HashSet<char> = s.into_iter().collect();

    let ans = if len == set.len() { "yes" } else { "no" };
    println!("{}", ans);
}
