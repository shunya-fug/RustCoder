use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let count = (1..=n)
        .map(|mut n| {
            let mut count = 0;
            while n > 0 && n % 2 == 0 {
                n /= 2;
                count += 1;
            }
            count
        })
        .position_max();
    let ans = match count {
        Some(x) => x + 1,
        None => 0,
    };

    println!("{}", ans);
}
