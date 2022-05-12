use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut count = 0;
    let mut target = 1;
    for a in &a {
        if *a == target {
            target += 1;
        } else {
            count += 1;
        }
    }

    if count == a.len() {
        println!("{}", -1);
    } else {
        println!("{}", count);
    }
}
