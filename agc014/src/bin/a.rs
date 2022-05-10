use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: (usize, usize, usize),
    }

    let mut pattern = vec![abc];
    let mut count = 0;
    loop {
        if [abc.0, abc.1, abc.2].iter().any(|x| x % 2 == 1) {
            break;
        }

        abc = (
            (abc.1 + abc.2) / 2,
            (abc.0 + abc.2) / 2,
            (abc.0 + abc.1) / 2,
        );
        count += 1;

        if pattern.contains(&abc) {
            count = -1;
            break;
        } else {
            pattern.push(abc);
        }
    }

    println!("{}", count);
}
