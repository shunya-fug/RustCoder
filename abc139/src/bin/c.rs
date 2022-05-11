use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: usize, h: [usize; n]}

    let mut ans = 0;
    let mut count = 0;
    for i in 0..n - 1 {
        if h[i + 1] <= h[i] {
            count += 1;
        } else {
            ans = ans.max(count);
            count = 0;
        }
    }
    ans = ans.max(count);

    println!("{}", ans);
}
