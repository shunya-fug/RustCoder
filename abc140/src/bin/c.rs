use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut b: [usize; n-1]
    }
    b.push(std::usize::MAX);
    let mut a = vec![0; n];
    a[0] = b[0];

    for (i, b) in b.windows(2).enumerate().rev() {
        a[i + 1] = *b.iter().min().unwrap();
    }

    println!("{}", a.into_iter().sum::<usize>());
}
