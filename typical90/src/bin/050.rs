use proconio::input;

fn main() {
    input! {n: usize, l: usize}

    let x = 1e9 as u128 + 7;
    let mut stair: Vec<u128> = vec![0; n + 1];
    stair[0] = 1;
    for i in 0..n {
        stair[i + 1] += stair[i];
        stair[i + 1] %= x;
        if i + l < stair.len() {
            stair[i + l] += stair[i] % x;
            stair[i + l] %= x;
        }
    }

    println!("{}", stair.last().unwrap());
}
