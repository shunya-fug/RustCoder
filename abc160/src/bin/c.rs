use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
        n: usize,
        mut a: [usize; n],
    }

    a.append(&mut a.iter().map(|x| *x + k).collect::<Vec<usize>>());
    let ans = (0..n).map(|i| a[i + n - 1] - a[i]).min().unwrap();

    println!("{}", ans);
}
