use itertools::iproduct;
use proconio::input;
use std::collections::HashMap;

fn vec2hashmap_mod46_counter(x: Vec<usize>) -> HashMap<usize, usize> {
    x.into_iter()
        .map(|x| x % 46)
        .fold(HashMap::new(), |mut hashmap, item| {
            let counter = hashmap.entry(item).or_insert(0);
            *counter += 1;
            hashmap
        })
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let a = vec2hashmap_mod46_counter(a);
    let b = vec2hashmap_mod46_counter(b);
    let c = vec2hashmap_mod46_counter(c);

    let ans = iproduct!(a.keys(), b.keys(), c.keys())
        .filter(|(x, y, z)| (*x + *y + *z) % 46 == 0)
        .fold(0, |acc, (x, y, z)| acc + a[x] * b[y] * c[z]);

    println!("{}", ans);
}
