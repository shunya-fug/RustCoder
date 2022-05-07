use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
        q: usize,
        b: [u64; q],
    }

    a.sort_unstable();
    a.dedup();
    for b in b {
        let complain = match a.binary_search(&b) {
            Ok(_) => 0,
            Err(0) => a[0] - b,
            Err(x) if x == a.len() => b - a[x - 1],
            Err(x) => (a[x] - b).min(b - a[x - 1]),
        };
        println!("{}", complain);
    }
}
