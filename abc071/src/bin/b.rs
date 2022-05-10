use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {mut s: Chars}
    s.sort_unstable();

    for c in (b'a'..=b'z').map(char::from) {
        if s.binary_search(&c).is_err() {
            println!("{}", c);
            return;
        }
    }
    println!("None");
}
