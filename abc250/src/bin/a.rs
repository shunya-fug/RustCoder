use ndarray::{s, Array, Array2};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, w, r, c): (usize, usize, usize, usize)
    }

    let mut arr: Array2<usize> = Array::zeros((h + 2, w + 2));
    arr.slice_mut(s![1..=h, 1..=w]).fill(1);

    let ans: usize = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
        .iter()
        .map(|(i, j)| arr[[*i, *j]])
        .sum();

    println!("{}", ans);
}
