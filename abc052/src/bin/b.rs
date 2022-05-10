use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {_: usize, s: Chars}
    let mut x = vec![0];

    for c in s {
        let current = *x.last().unwrap();
        match c {
            'I' => x.push(current + 1),
            'D' => x.push(current - 1),
            _ => continue,
        }
    }

    println!("{}", x.iter().max().unwrap());
}
