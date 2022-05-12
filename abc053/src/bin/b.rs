use std::collections::VecDeque;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {s: Chars}
    let mut s = VecDeque::from(s);

    while s.pop_front().unwrap() != 'A' {}
    while s.pop_back().unwrap() != 'Z' {}

    println!("{}", s.len() + 2);
}
