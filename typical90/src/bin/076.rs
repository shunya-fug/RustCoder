use proconio::input;

fn main() {
    input! {n: usize, a: [usize; n],}

    let a: Vec<usize> = a
        .repeat(2)
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();

    if a[n - 1] / 10 * 10 != a[n - 1] {
        println!("No");
        return;
    }
    for i in 0..n {
        let l = a[i];
        let r = l + a[n - 1] / 10;
        if a.binary_search(&r).is_ok() {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
