use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: String,
        b: String,
    }

    let ab: usize = (a + &b).parse().unwrap();
    for i in 0..ab / 2 {
        if i * i == ab {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
