use proconio::input;

fn main() {
    input! {n: usize, k: usize}

    const MOD: usize = 1e9 as usize + 7;
    let mut ans = match n {
        1 => k,
        _ => k * (k - 1) % MOD,
    };

    if n >= 3 {
        let mut n = n - 2;
        let mut k = (k - 2).max(0);
        while n > 0 {
            if n & 1 == 1 {
                ans *= k;
                ans %= MOD;
            }

            k = k * k % MOD;
            n >>= 1;
        }
    }

    println!("{}", ans);
}
