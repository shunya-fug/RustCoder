use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u128; 3 * n],
    }
    a.sort_unstable();

    let ans: u128 = a[n..].iter().step_by(2).sum();

    println!("{}", ans);
}
