use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, w): (usize, usize),
        mut a: [usize; n]
    }
    a.sort_unstable();

    let mut good = vec![false; w + 1];
    for n in &a {
        if let Some(v) = good.get_mut(*n) {
            *v = true;
        }
    }
    for c in a.iter().combinations(2) {
        let n: usize = c.into_iter().sum();
        if let Some(v) = good.get_mut(n) {
            *v = true;
        }
    }
    for c in a.iter().combinations(3) {
        let n: usize = c.into_iter().sum();
        if let Some(v) = good.get_mut(n) {
            *v = true;
        }
    }

    let ans = good.into_iter().filter(|v| *v).count();
    println!("{}", ans);
}
