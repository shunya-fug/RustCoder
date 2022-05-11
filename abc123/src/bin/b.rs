use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        dish: [usize; 5],
    }

    let mut ans = std::usize::MAX;
    for order in (0..5).permutations(5) {
        let mut t = 0;
        for i in &order[..4] {
            t += if dish[*i] % 10 == 0 {
                dish[*i]
            } else {
                10 * (dish[*i] / 10 + 1)
            }
        }
        t += dish[*order.last().unwrap()];
        ans = ans.min(t);
    }

    println!("{}", ans);
}
