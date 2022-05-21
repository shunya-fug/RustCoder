use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [usize; n]
    }

    let mut ans = 0;
    for mut a in x {
        while a % 2 == 0 {
            ans += 1;
            a /= 2;
        }
    }

    println!("{}", ans);
}
