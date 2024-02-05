use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Pos3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos3D {
    pub const ZERO: Self = Pos3D::new(0, 0, 0);

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn clip_behind_player(&self, pos1: Pos3D) -> Pos3D {
        let da = self.y as f64;
        let db = pos1.y as f64;

        let mut d = da - db;
        if d == 0.0 {
            d = 1.0;
        }
        let s = da / d;

        let mut p = Pos3D::new(
            (self.x as f64 + s * (pos1.x - self.x) as f64) as i32,
            (self.y as f64 + s * (pos1.y - self.y) as f64) as i32,
            (self.z as f64 + s * (pos1.z - self.z) as f64) as i32,
        );

        // Prevent clipping
        if p.y == 0 {
            p.y = 1
        };

        p
    }
}

impl Add<Pos3D> for Pos3D {
    type Output = Pos3D;

    fn add(self, rhs: Pos3D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
