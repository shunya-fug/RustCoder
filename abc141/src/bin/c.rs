use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k, q): (usize, isize, usize),
        a: [usize; q],
    }

    let mut participant: Vec<isize> = vec![k - q as isize; n];
    for a in a {
        participant[a - 1] += 1;
    }

    for p in participant {
        let result = if p > 0 { "Yes" } else { "No" };
        println!("{}", result);
    }
}
