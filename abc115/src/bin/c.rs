use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        mut h: [usize; n]
    }
    h.sort_unstable();

    let ans: usize = h
        .windows(k)
        .map(|section| *section.last().unwrap() - *section.first().unwrap())
        .min()
        .unwrap();

    println!("{}", ans);
}
