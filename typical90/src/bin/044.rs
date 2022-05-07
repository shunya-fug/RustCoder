use std::collections::VecDeque;

use proconio::{derive_readable, input};

#[derive_readable]
struct Query {
    t: usize,
    x: usize,
    y: usize,
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        query: [Query; q],
    }

    let mut a = VecDeque::from(a);
    for query in query {
        match query.t {
            1 => a.swap(query.x - 1, query.y - 1),
            2 => a.rotate_right(1),
            3 => println!("{}", a[query.x - 1]),
            _ => panic!(),
        }
    }
}
