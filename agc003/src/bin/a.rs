use im_rc::HashSet;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {s: Chars}
    let direction: HashSet<char> = s.into_iter().collect();

    let ans = if direction.len() % 2 == 0
        && ((direction.contains(&'N') && direction.contains(&'S'))
            || (direction.contains(&'W') && direction.contains(&'E')))
    {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
