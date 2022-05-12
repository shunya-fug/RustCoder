use itertools::{Itertools, MinMaxResult};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        (t, a): (f64, f64),
        h: [f64; n],
    }

    let t = h.into_iter().map(|x| t - 0.006 * x).map(|t| (t - a).abs());
    let ans = match t.position_minmax() {
        MinMaxResult::MinMax(x, _) => x + 1,
        _ => panic!(),
    };

    println!("{}", ans);
}
