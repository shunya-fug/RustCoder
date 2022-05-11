use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {s: Chars}

    let target1: Vec<char> = ['0', '1'].iter().cycle().take(s.len()).cloned().collect();
    let target2: Vec<char> = ['1', '0'].iter().cycle().take(s.len()).cloned().collect();

    let change1 = target1.iter().zip(&s).filter(|(a, b)| a != b).count();
    let change2 = target2.iter().zip(&s).filter(|(a, b)| a != b).count();

    println!("{}", change1.min(change2));
}
