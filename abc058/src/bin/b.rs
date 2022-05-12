use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {(o, e): (Chars, Chars)}

    let ans: String = o.into_iter().interleave(e).collect();
    println!("{}", ans);
}
