use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {(a, b): (f64, f64)}

    let mut ans: f64 = -1.0;
    for n in 1..=1000 {
        let n = n as f64;
        if (n * 1.08).floor() == (n + a) && (n * 1.1).floor() == (n + b) {
            ans = n;
            break;
        }
    }

    println!("{}", ans);
}
