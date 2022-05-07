use ndarray::{s, Array, Array2};
use std::iter::FromIterator;

fn main() {
    proconio::input! {
        h: usize,
        w: usize,
    }

    let mut light: Array2<bool> = Array::from_elem((h + 2, w + 2), false);

    for i in 1..=h {
        for j in 1..=w {
            if Array::from_iter(light.slice(s![i - 1..=i + 1, j - 1..=j + 1]))
                .iter()
                .all(|&&x| !x)
            {
                light[[i, j]] = true
            }
        }
    }

    let light = light.slice(s![1..=h, 1..=w]);
    let num = Array::from_iter(light.iter())
        .iter()
        .filter(|&&&x| x)
        .count();

    print!("{}", num)
}
