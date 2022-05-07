use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[u128; 6]; n],
    }

    let x = 1e9 as u128 + 7;
    let ans: u128 = a
        .into_iter()
        .map(|a| a.iter().sum::<u128>())
        .fold(1, |acc, item| acc * item % x);
    println!("{}", ans);
}
