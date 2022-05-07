use proconio::derive_readable;
use std::collections::VecDeque;

#[derive_readable]
struct Query {
    t: usize,
    x: usize,
}

impl Query {
    fn execute(&self, mut deck: VecDeque<usize>) -> VecDeque<usize> {
        match self.t {
            1 => deck.push_front(self.x),
            2 => deck.push_back(self.x),
            3 => {
                println!("{}", deck[self.x - 1]);
            }
            _ => (),
        }
        deck
    }
}

fn main() {
    proconio::input! {
        q: usize,
    }

    let mut deck = VecDeque::new();
    for _ in 0..q {
        proconio::input! {query: Query}
        deck = query.execute(deck);
    }
}
