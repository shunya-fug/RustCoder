use proconio::input;

fn main() {
    input! {mut n: usize,}

    let mut factor: Vec<usize> = Vec::new();
    let m = n;
    for i in 2..m {
        if i * i > n {
            break;
        }
        while n % i == 0 {
            n /= i;
            factor.push(i);
        }
    }
    if n != 1 {
        factor.push(n)
    }

    let mut ans = 0;
    let mut ball = 1;
    while ball < factor.len() {
        ans += 1;
        ball *= 2;
    }

    println!("{}", ans);
}
