use num::pow;
use proconio::input;

fn main() {
    input! {mut l: usize, r: usize, }
    const MOD: usize = 1e9 as usize + 7;

    let mut counter = vec![0; 19];
    let mut length = vec![0; 19];
    for i in 0..counter.len() {
        let n = pow(10, i);
        length[i] = n.to_string().len();
        if l < n && r < n {
            counter[i - 1] = r - l;
            break;
        } else if l < n && n < r {
            counter[i - 1] = n - l;
            l = n;
        }
    }

    let mut ans = 0;
    for (length, count) in length.into_iter().zip(counter) {
        ans += length * (count % MOD);
        ans %= MOD;
    }

    println!("{}", ans);
}
