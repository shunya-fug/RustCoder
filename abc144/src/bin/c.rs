use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {n: usize}

    let mut ans = std::usize::MAX;
    for i in 1..n {
        if i * i > n {
            break;
        }

        let j = n / i;
        if i * j == n {
            ans = ans.min((i - 1) + (j - 1));
        }
    }

    println!("{}", ans);
}
