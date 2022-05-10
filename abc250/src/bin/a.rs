use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, w, r, c): (usize, usize, usize, usize)
    }

    let ans = if h == 1 && w == 1 {
        0
    } else if h == 1 {
        if c == 1 || c == w {
            1
        } else {
            2
        }
    } else if w == 1 {
        if r == 1 || r == h {
            1
        } else {
            2
        }
    } else if r == 1 && c == 1 || r == 1 && c == w || r == h && c == 1 || r == h && c == w {
        2
    } else if r == 1 || r == h || c == 1 || c == w {
        3
    } else {
        4
    };

    println!("{}", ans);
}
