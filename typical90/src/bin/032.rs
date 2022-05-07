use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        xy: [(usize, usize); m],
    }

    let xy: HashSet<(usize, usize)> = xy.into_iter().map(|(x, y)| (x - 1, y - 1)).collect();

    let mut ans = std::usize::MAX;
    for runner in (0..n).permutations(n) {
        if runner
            .iter()
            .zip(&runner[1..])
            .all(|(x, y)| !xy.contains(&(*x.min(y), *x.max(y))))
        {
            let time = runner.into_iter().enumerate().map(|(j, i)| a[i][j]).sum();
            ans = ans.min(time);
        }
    }

    if ans == std::usize::MAX {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
