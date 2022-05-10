use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, a, b): (usize, usize, usize)
    }

    let mut color = ['.', '#'];
    for _ in 0..n {
        for _ in 0..a {
            let mut row = String::new();
            for j in 0..n {
                for _ in 0..b {
                    row.push(color[j % 2]);
                }
            }
            println!("{}", row);
        }
        color.reverse();
    }
}
