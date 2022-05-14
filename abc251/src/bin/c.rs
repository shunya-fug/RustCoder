use proconio::{derive_readable, fastout, input};
use std::collections::HashSet;

#[derive_readable]
struct Poem {
    s: String,
    t: usize,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        poem: [Poem; n],
    }

    let mut ans = 0;
    let mut best = &poem[0];
    let mut submitted: HashSet<String> = HashSet::new();
    for (i, poem) in poem.iter().enumerate() {
        if !submitted.contains(&poem.s) {
            // original
            if best.t < poem.t {
                best = poem;
                ans = i;
            }
        }
        submitted.insert(poem.s.clone());
    }

    println!("{}", ans + 1);
}
