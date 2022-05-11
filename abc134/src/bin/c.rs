use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: usize, mut a: [usize; n]}
    let mut a_sorted = a.clone();
    a_sorted.sort_unstable();
    let amax = a_sorted.pop().unwrap();
    if a.iter().filter(|a| **a == amax).count() == 1 {
        let amax2nd = a_sorted.pop().unwrap();
        for a in a {
            println!("{}", if a == amax { amax2nd } else { amax });
        }
    } else {
        for _ in a {
            println!("{}", amax);
        }
    }
}
