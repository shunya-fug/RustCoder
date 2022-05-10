use proconio::{derive_readable, fastout, input};

#[derive_readable]
struct Coupon {
    x: usize,
    y: usize,
    c: usize,
}

#[fastout]
fn main() {
    input! {
        (a, b, m): (usize, usize, usize),
        pa: [usize; a],
        pb: [usize; b],
        coupon: [Coupon; m],
    }

    let mut ans = pa.iter().min().unwrap() + pb.iter().min().unwrap();
    for coupon in coupon {
        ans = ans.min(pa[coupon.x - 1] + pb[coupon.y - 1] - coupon.c);
    }

    println!("{}", ans);
}
