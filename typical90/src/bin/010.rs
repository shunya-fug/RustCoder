use proconio::{derive_readable, input};

#[derive_readable]
struct Student {
    class: usize,
    point: usize,
}

fn main() {
    input! {
        n: usize,
        student: [Student; n],
    }
    let acc_class = |class| {
        let mut acc = student
            .iter()
            .scan(0, |state, s| {
                if s.class == class {
                    *state += s.point;
                }
                Some(*state)
            })
            .collect::<Vec<usize>>();
        acc.insert(0, 0);
        acc
    };
    let acc_class_1 = acc_class(1);
    let acc_class_2 = acc_class(2);

    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {l: usize, r: usize}
        let a = acc_class_1[r] - acc_class_1[l - 1];
        let b = acc_class_2[r] - acc_class_2[l - 1];
        println!("{} {}", a, b);
    }
}
