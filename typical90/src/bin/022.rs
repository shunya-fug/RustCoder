use num_integer::gcd;
use proconio::input;

fn main() {
    input! {a: usize, b: usize, c: usize}

    let r = gcd(a, gcd(b, c));
    let count = (a / r - 1) + (b / r - 1) + (c / r - 1);

    println!("{}", count);
}
