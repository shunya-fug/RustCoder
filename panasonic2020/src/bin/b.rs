use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {(h, w): (usize, usize)}

    let ans = if h == 1 || w == 1 {
        1
    } else {
        let mut ans = w * h / 2;
        if h % 2 == 1 && w % 2 == 1 {
            ans += 1;
        }
        ans
    };

    println!("{}", ans);
}
