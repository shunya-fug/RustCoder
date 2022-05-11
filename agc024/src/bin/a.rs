use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {(a, b, _, k): (i128, i128, i128, i128)}

    println!("{}", if k % 2 == 1 { -(a - b) } else { a - b });
}
