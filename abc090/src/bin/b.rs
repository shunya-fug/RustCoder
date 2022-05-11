use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {(a, b): (usize, usize)}

    let ans = (a..=b)
        .filter(|n| {
            let s = n.to_string();
            s == s.chars().rev().collect::<String>()
        })
        .count();

    println!("{}", ans);
}
