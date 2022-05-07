use num::pow;

fn base_8_to_10(n_8: u128) -> u128 {
    let n_str = n_8.to_string();
    let mut n_10 = 0;
    for (e, x) in n_str.chars().rev().enumerate() {
        n_10 += (x as u128 - '0' as u128) * pow(8, e);
    }
    n_10
}

fn base_10_to_9(mut n_10: u128) -> u128 {
    let mut n_9_str = String::new();
    while n_10 > 0 {
        n_9_str.push(std::char::from_digit((n_10 % 9) as u32, 10).unwrap());
        n_10 /= 9;
    }

    n_9_str
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u128>()
        .unwrap_or(0)
}

fn rewrite_8_to_5(n: u128) -> u128 {
    n.to_string().replace('8', "5").parse::<u128>().unwrap()
}

fn main() {
    proconio::input! {
        mut n: u128,
        k: usize,
    }

    for _ in 0..k {
        n = base_8_to_10(n);
        n = base_10_to_9(n);
        n = rewrite_8_to_5(n);
    }

    println!("{}", n);
}
