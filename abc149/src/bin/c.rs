use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    for n in x.. {
        if !(2..n).take_while(|i| i * i <= n).any(|i| n % i == 0) {
            println!("{}", n);
            break;
        }
    }
}
