use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
        lr: [(usize, usize); m],
    }

    let mut l_max = 0;
    let mut r_min = n;
    for (l, r) in lr {
        l_max = l_max.max(l);
        r_min = r_min.min(r);
    }

    println!("{}", (r_min + 1).saturating_sub(l_max));
}
