use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {s: Chars}

    let mut ans = "AC".to_string();

    if s.first() != Some(&'A') {
        ans = "WA".to_string();
    }
    if s[2..=s.len() - 2].iter().filter(|c| **c == 'C').count() != 1 {
        ans = "WA".to_string();
    }
    if !s
        .into_iter()
        .all(|c| c == 'A' || c == 'C' || ('a'..='z').contains(&c))
    {
        ans = "WA".to_string();
    }

    println!("{}", ans);
}
