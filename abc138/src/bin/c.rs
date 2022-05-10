use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut v: [f64; n],
    }

    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let ans = v.iter().fold(v[0], |acc, x| (acc + x) / 2.0);

    println!("{}", ans);
}
