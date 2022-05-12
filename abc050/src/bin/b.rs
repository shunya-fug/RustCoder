use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: [isize; n],
        m: usize,
        px: [(usize, isize); m],
    }

    let solve_all_problem: isize = t.iter().sum();
    for (p, x) in px {
        let result = solve_all_problem - (t[p - 1] - x);
        println!("{}", result);
    }
}
