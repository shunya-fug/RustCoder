use num::pow;
use proconio::{fastout, input};
use std::vec;

#[fastout]
fn main() {
    input! {x: usize}
    let mut perfect_power = vec![false; x + 1];
    perfect_power[1] = true;
    for i in 2..x {
        if i * 2 > x {
            break;
        }

        for p in 2.. {
            let n = pow(i, p);
            if n > x {
                break;
            }
            perfect_power[n] = true;
        }
    }

    for n in (1..=x).rev() {
        if perfect_power[n] {
            println!("{}", n);
            break;
        }
    }
}
