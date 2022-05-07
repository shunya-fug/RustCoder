use itertools::{iproduct, Itertools};
use ndarray::{s, Array, Array2};
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[isize; w]; h],
        b: [[isize; w]; h],
    }

    let a: Array2<isize> =
        Array::from_shape_vec((h, w), a.into_iter().flatten().collect()).unwrap();
    let b: Array2<isize> =
        Array::from_shape_vec((h, w), b.into_iter().flatten().collect()).unwrap();

    let mut diff = b - a;
    let mut count = 0;
    for (i, j) in iproduct!(0..h - 1, 0..w - 1) {
        let d = diff[[i, j]];
        diff.slice_mut(s![i..=i + 1, j..=j + 1])
            .mapv_inplace(|x| x - d);
        count += d.abs();
    }

    if diff.into_iter().all_equal() {
        println!("Yes\n{}", count);
    } else {
        println!("No");
    }
}
