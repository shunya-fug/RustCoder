use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {s: Chars}
    const TARGET: usize = 753;

    let mut ans = std::usize::MAX;
    for chs in s.windows(3) {
        let n = chs.iter().collect::<String>().parse::<usize>().unwrap();
        ans = ans.min(n.max(TARGET) - n.min(TARGET));
    }

    println!("{}", ans);
}
