use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (isize, isize),
    }

    let ans = (n % k).min(-(n % k - k));
    println!("{}", ans);
}
