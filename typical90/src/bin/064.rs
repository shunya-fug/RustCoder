use proconio::{derive_readable, input};

#[derive_readable]
struct Query {
    l: usize,
    r: usize,
    v: isize,
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [isize; n],
        query: [Query; q],
    }

    let mut ans = 0;
    let mut inconvenience: Vec<isize> = a
        .windows(2)
        .map(|x| {
            ans += (x[0] - x[1]).abs();
            x[0] - x[1]
        })
        .collect();
    for query in query {
        if inconvenience.get(query.l - 2).is_some() {
            inconvenience[query.l - 1] += query.v;
        }
        if inconvenience.get(query.r).is_some() {
            inconvenience[query.r - 1] -= query.v;
        }

        todo!()
    }
}
