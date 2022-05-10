use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m, x): (usize, usize, usize),
        a: [usize; m],
    }

    let mut map: Vec<usize> = vec![0; n + 1];
    for a in a {
        map[a] = 1;
    }

    let left_cost: usize = map[..x].iter().sum();
    let right_cost: usize = map[x..].iter().sum();
    let ans = left_cost.min(right_cost);

    println!("{}", ans);
}
