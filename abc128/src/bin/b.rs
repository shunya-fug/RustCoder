use itertools::Itertools;
use proconio::{fastout, input};
use std::cmp::Ordering::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        restaurant: [(String, usize); n],
    }
    let mut restaurant = restaurant
        .into_iter()
        .enumerate()
        .map(|(i, r)| (r.0, r.1, i + 1))
        .collect_vec();
    restaurant.sort_by(|a, b| match a.0.cmp(&b.0) {
        Less | Greater => a.0.cmp(&b.0),
        Equal => a.1.cmp(&b.1).reverse(),
    });

    for (_, _, i) in restaurant.iter() {
        println!("{}", i);
    }
}
