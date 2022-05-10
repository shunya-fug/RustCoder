use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    }

    d.sort_unstable();
    let last_of_first_half = d[n / 2 - 1];
    let first_of_second_half = d[n / 2];

    let ans = if last_of_first_half == first_of_second_half {
        0
    } else {
        first_of_second_half - last_of_first_half
    };

    println!("{}", ans);
}
