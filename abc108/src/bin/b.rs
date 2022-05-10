use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p1: (isize, isize),
        p2: (isize, isize),
    }

    let dx = p2.0 - p1.0;
    let dy = p2.1 - p1.1;

    let ans = format!("{} {} {} {}", p2.0 - dy, p2.1 + dx, p1.0 - dy, p1.1 + dx);

    println!("{}", ans);
}
