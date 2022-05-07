use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        point: [(usize, usize); n],
    }

    let mut point: Vec<usize> = point
        .into_iter()
        .flat_map(|(a, b)| vec![a - b, b])
        .collect();
    point.sort_unstable();
    point.reverse();

    let ans: usize = point[..k].iter().sum();
    println!("{}", ans);
}
