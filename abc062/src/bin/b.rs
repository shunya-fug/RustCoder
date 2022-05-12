use itertools::Itertools;
use ndarray::{s, Array, Array2, Axis};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        a: [String; h],
    }

    let mut base: Array2<char> = Array::from_elem((h + 2, w + 2), '#');
    let a: Array2<char> = Array::from_shape_vec(
        (h, w),
        a.into_iter()
            .flat_map(|s| s.chars().collect_vec())
            .collect_vec(),
    )
    .unwrap();

    base.slice_mut(s![1..=h, 1..=w]).assign(&a);
    let ans = base;

    for row in ans.axis_iter(Axis(0)) {
        println!("{}", row.iter().collect::<String>());
    }
}
