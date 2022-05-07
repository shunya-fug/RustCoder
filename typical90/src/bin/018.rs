use num::pow;
use proconio::input;
use std::f64::consts::PI;

#[derive(Default)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

struct FerrisWheel {
    t: f64,
    r: f64,
    position: Position,
}

struct Statue {
    position: Position,
}

impl Position {
    fn angle2(&self, other: &Position) -> f64 {
        (self.z - other.z)
            .atan2((pow(self.x - other.x, 2) + pow(self.y - other.y, 2)).sqrt())
            .to_degrees()
    }
}

impl FerrisWheel {
    fn new(t: f64, r: f64) -> FerrisWheel {
        FerrisWheel {
            t,
            r,
            position: Default::default(),
        }
    }

    fn rotate(&mut self, t: f64) {
        // self.t: 周期, t: 経過時間
        let theta = 2.0 * PI * (t / self.t);
        self.position.y = -self.r * theta.sin();
        self.position.z = self.r - self.r * theta.cos();
    }
}

impl Statue {
    fn new(x: f64, y: f64) -> Statue {
        Statue {
            position: Position { x, y, z: 0.0 },
        }
    }
}

fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: isize,
        e: [f64; q],
    }
    let mut ferris_wheel = FerrisWheel::new(t, l / 2.0);
    let statue = Statue::new(x, y);

    for e in e {
        ferris_wheel.rotate(e);
        let angle = ferris_wheel.position.angle2(&statue.position);
        println!("{:}", angle);
    }
}
