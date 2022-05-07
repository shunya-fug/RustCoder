use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut abc: [usize; 3],
    }

    abc.sort_unstable();
    abc.reverse();
    let (a, b, c) = abc.into_iter().collect_tuple().unwrap();

    let mut ans = 10000;
    for i in 0..=(n / a) {
        for j in 0..=((n - a * i) / b) {
            let sum = a * i + b * j;
            if (n - sum) % c == 0 {
                ans = ans.min(i + j + (n - sum) / c);
            }
        }
    }

    println!("{}", ans);
}
