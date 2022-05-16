use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        ps: [(usize, String); m]
    }
    let mut q: Vec<usize> = vec![0; n];
    let mut ac: Vec<bool> = vec![false; n];

    for (p, s) in ps {
        if !ac[p - 1] {
            if s == "AC" {
                ac[p - 1] = true;
            }
            if s == "WA" {
                q[p - 1] += 1;
            }
        }
    }

    let correct = ac.iter().filter(|x| **x).count();
    let penalty: usize = q
        .into_iter()
        .zip(ac)
        .filter(|(_, ac)| *ac)
        .map(|(q, _)| q)
        .sum();

    println!("{} {}", correct, penalty);
}
