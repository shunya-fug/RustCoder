use ndarray::{Array, Array2, Axis};
use proconio::{fastout, input};

fn bingo(a: &Array2<usize>) -> bool {
    if a.axis_iter(Axis(0)).any(|row| row.iter().all(|x| *x == 0)) {
        return true;
    }
    if a.axis_iter(Axis(1)).any(|col| col.iter().all(|x| *x == 0)) {
        return true;
    }
    if a.diag().iter().all(|x| *x == 0) {
        return true;
    }
    if a[[0, 2]] == 0 && a[[1, 1]] == 0 && a[[2, 0]] == 0 {
        return true;
    }

    false
}

#[fastout]
fn main() {
    input! {
        a: [[usize; 3]; 3],
        n: usize,
        b: [usize; n],
    }
    let mut a: Array2<usize> =
        Array::from_shape_vec((3, 3), a.into_iter().flatten().collect()).unwrap();

    for b in b {
        a.mapv_inplace(|x| if x == b { 0 } else { x });
    }

    println!("{}", if bingo(&a) { "Yes" } else { "No" });
}
