use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: usize}

    let mut lucas_number: Vec<u128> = vec![0; n + 1];
    lucas_number[0] = 2;
    lucas_number[1] = 1;

    for i in 2..=n {
        lucas_number[i] = lucas_number[i - 1] + lucas_number[i - 2];
    }

    println!("{}", lucas_number.last().unwrap());
}
