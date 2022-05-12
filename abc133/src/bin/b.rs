use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, d): (usize, usize),
        point: [[i32; d]; n],
    }

    let base = |x: &Vec<i32>, y: &Vec<i32>| -> i32 {
        x.iter().zip(y).map(|(a, b)| (a - b) * (a - b)).sum::<i32>()
    };

    let mut count = 0;
    for p in point.iter().combinations(2) {
        let target = base(p[0], p[1]);
        for i in 1.. {
            if i * i == target {
                count += 1;
                break;
            }
            if i * i > target {
                break;
            }
        }
    }

    println!("{}", count);
}
