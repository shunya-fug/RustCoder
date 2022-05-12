use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, _): (usize, usize),
        c: [String; h],
    }

    for c in c {
        println!("{}", c);
        println!("{}", c);
    }
}
