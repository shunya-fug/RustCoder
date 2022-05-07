use num_integer::lcm;
use proconio::input;

fn main() {
    input! {
        a: u128, b:u128
    }
    let ans = lcm(a, b);

    if ans > 1e18 as u128 {
        println!("Large");
    } else {
        println!("{}", ans);
    }
}
