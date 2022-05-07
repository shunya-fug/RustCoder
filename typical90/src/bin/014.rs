use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort_unstable();
    b.sort_unstable();

    let ans: usize = a.iter().zip(b).map(|(a, b)| a.max(&b) - a.min(&b)).sum();
    println!("{}", ans);
}
