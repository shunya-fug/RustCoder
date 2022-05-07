use num::pow;
use proconio::input;
use std::mem::swap;

fn modinv(a: &i128, m: &i128) -> i128 {
    // 逆元を求める関数
    // ref -> https://qiita.com/drken/items/3b4fdf0a78e7a138cd9a#3-5-%E6%8B%A1%E5%BC%B5-euclid-%E3%81%AE%E4%BA%92%E9%99%A4%E6%B3%95%E3%81%AB%E3%82%88%E3%82%8B%E9%80%86%E5%85%83%E8%A8%88%E7%AE%97
    let (mut a, mut b, mut u, mut v) = (*a, *m, 1, 0);

    while b > 0 {
        let t = a / b;
        a -= t * b;
        u -= t * v;
        swap(&mut a, &mut b);
        swap(&mut u, &mut v);
    }

    if u < 0 {
        u + m
    } else {
        u
    }
}

fn main() {
    input! {mut l: usize, r: usize, }
    const MOD: i128 = 1e9 as i128 + 7;

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

    for i in 1..13 {
        println!("{:}", modinv(&i, &13));
    }
}
