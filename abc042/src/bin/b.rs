use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, _): (usize, usize),
        mut s: [String; n]
    }
    s.sort();
    let ans = s.join("");

    println!("{}", ans);
}
