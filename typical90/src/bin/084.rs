use proconio::input;
use proconio::marker::Chars;

fn rle(s: Vec<char>) -> Vec<(char, usize)> {
    let mut res: Vec<(char, usize)> = Vec::new();
    let mut pre = ' ';
    let mut chain = 1;
    for c in s.into_iter() {
        if c == pre {
            chain += 1;
        } else {
            if pre != ' ' {
                res.push((pre, chain));
            }
            pre = c;
            chain = 1;
        }
    }
    res.push((pre, chain));
    res
}

fn main() {
    input! {n: usize, s: Chars}
    let s = rle(s);

    let ret: usize = s.into_iter().map(|(_, n)| n * (n + 1) / 2).sum();
    let ans = n * (n + 1) / 2 - ret;

    println!("{}", ans);
}
