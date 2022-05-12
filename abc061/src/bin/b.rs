use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        ab: [(usize, usize); m],
    }
    let mut road = vec![0; n + 1];
    for (a, b) in ab {
        road[a] += 1;
        road[b] += 1;
    }

    for road in &road[1..] {
        println!("{}", road);
    }
}
