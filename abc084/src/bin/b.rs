use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
        s: Chars,
    }

    let ans = if s[a] == '-' {
        let left = &s[..a];
        let right = &s[a + 1..];

        if left.len() == a
            && right.len() == b
            && left.iter().all(|c| ('0'..='9').contains(c))
            && right.iter().all(|c| ('0'..='9').contains(c))
        {
            "Yes"
        } else {
            "No"
        }
    } else {
        "No"
    };

    println!("{}", ans);
}
