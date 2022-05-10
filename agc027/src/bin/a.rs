use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, mut x): (usize, usize),
        mut a: [usize; n],
    }

    a.sort_unstable();
    let mut count = 0;
    for a in &a[..n - 1] {
        if *a > x {
            break;
        }
        x -= a;
        count += 1;
    }
    if a.last().unwrap() == &x {
        count += 1;
    }

    println!("{}", count);
}
