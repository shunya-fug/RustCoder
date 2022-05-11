use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {(a, b): (isize, isize)}

    if a <= 0 && 0 <= b {
        println!("Zero");
    } else {
        let negative: usize = (a..=b).filter(|x| *x < 0).count();
        if negative % 2 == 0 {
            println!("Positive");
        } else {
            println!("Negative");
        }
    }
}
