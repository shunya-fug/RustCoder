use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, d, x): (usize, usize, usize),
        a: [usize; n],
    }

    let mut count = x;
    for a in a {
        let mut next = 0;
        for d in 0..d {
            if d == next {
                count += 1;
                next += a;
            }
        }
    }

    println!("{}", count);
}
