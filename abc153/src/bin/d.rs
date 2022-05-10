use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {mut h: usize}

    let mut attack: u128 = 0;
    let mut monster: u128 = 1;
    while h > 0 {
        attack += monster;
        monster *= 2;
        h /= 2;
    }

    println!("{}", attack);
}
