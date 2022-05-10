use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {(n, a, b): (usize, usize, usize)}

    let mut blue = a * (n / (a + b));
    let m = n - (n / (a + b)) * (a + b);
    blue += a.min(m);

    println!("{}", blue);
}
