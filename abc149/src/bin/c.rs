use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    for n in x.. {
        for i in 2..n {
            if i * i > n {
                break;
            }
        }
    }
}
