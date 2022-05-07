use itertools::Itertools;
use proconio::input;

fn main() {
    input! {n: usize}

    let mut parentheses: Vec<String> = Vec::new();
    for bit in 0..(1 << n) {
        let s = (0..n)
            .map(|digit| if (bit & (1 << digit)) == 0 { '(' } else { ')' })
            .join("");
        if s.chars().filter(|c| *c == '(').count() == s.chars().filter(|c| *c == ')').count()
            && s.chars()
                .scan(0, |state, c| {
                    *state += if c == '(' { 1 } else { -1 };
                    Some(*state)
                })
                .all(|x| x >= 0)
        {
            parentheses.push(s);
        }
    }

    parentheses.sort();
    for s in parentheses {
        println!("{}", s);
    }

    let n = 3;
    for bit in 0..(1 << n) {
        let s = (0..n)
            .map(|digit| if (bit & (1 << digit)) == 0 { '(' } else { ')' })
            .collect::<String>();
        println!("{}", s);
    }
}
