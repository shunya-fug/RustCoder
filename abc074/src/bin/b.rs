use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: [usize; n],
    }

    let mut ans = 0;
    for x in x {
        ans += x.min(k - x) * 2;
    }

    println!("{}", ans);
}
