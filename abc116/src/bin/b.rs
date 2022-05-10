use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {s: usize}

    let f = |n: usize| if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
    let mut a: Vec<usize> = Vec::new();

    for i in 0..=100_0000 {
        if i == 0 {
            a.push(s);
        } else {
            let v = f(a[i - 1]);

            if a.contains(&v) {
                println!("{}", i + 1);
                return;
            } else {
                a.push(v);
            }
        }
    }
}
