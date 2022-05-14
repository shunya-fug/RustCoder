use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {s: String}

    let ans = match s.len() {
        1 => s.repeat(6),
        2 => s.repeat(3),
        3 => s.repeat(2),
        _ => panic!(),
    };

    println!("{}", ans);
}
