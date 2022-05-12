use std::collections::HashSet;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {n: usize, w: [Chars; n]}
    let w_set: HashSet<Vec<char>> = w.iter().cloned().collect();

    let mut ans = "Yes";
    if w_set.len() != w.len() {
        ans = "No";
    } else {
        let mut last = *w[0].first().unwrap();

        for w in w {
            if last != *w.first().unwrap() {
                ans = "No";
                break;
            }
            last = *w.last().unwrap();
        }
    }

    println!("{}", ans);
}
