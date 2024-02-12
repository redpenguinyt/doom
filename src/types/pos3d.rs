use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Pos3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos3D {
    pub const ZERO: Self = Self::new(0, 0, 0);

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn clip_behind_player(&self, pos1: Self) -> Self {
        let da = f64::from(self.y);
        let db = f64::from(pos1.y);

        let mut d = da - db;
        if d == 0.0 {
            d = 1.0;
        }
        let s = da / d;

        let mut p = Self::new(
            s.mul_add(f64::from(pos1.x - self.x), f64::from(self.x)) as i32,
            s.mul_add(f64::from(pos1.y - self.y), f64::from(self.y)) as i32,
            s.mul_add(f64::from(pos1.z - self.z), f64::from(self.z)) as i32,
        );

        // Prevent clipping
        if p.y == 0 {
            p.y = 1;
        };

        p
    }
}

impl Add<Self> for Pos3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
