use itertools::Itertools;
use ndarray::{Array, Array2};
use proconio::fastout;

#[fastout]
fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }
    let a: Array2<usize> =
        Array::from_shape_vec((h, w), a.into_iter().flatten().collect()).unwrap();

    let sum_row = (0..h).map(|i| a.row(i).sum()).collect_vec();
    let sum_col = (0..w).map(|j| a.column(j).sum()).collect_vec();

    for i in 0..h {
        let b = (0..w)
            .map(|j| sum_row[i] + sum_col[j] - a[[i, j]])
            .join(" ");
        println!("{}", b);
    }
}
