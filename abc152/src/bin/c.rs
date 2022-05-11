use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: usize, p: [usize; n]}

    let mut min = std::usize::MAX;
    let mut count = 0;
    for p in p {
        if p <= min {
            count += 1;
            min = p;
        }
    }

    println!("{}", count);
}
