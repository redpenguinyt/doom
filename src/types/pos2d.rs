use sdl2::rect::Point;

#[derive(Debug, Clone, Copy)]
pub struct Pos2D {
    pub x: i32,
    pub y: i32,
}

impl Pos2D {
    pub const ZERO: Self = Self::new(0, 0);

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt().round()
    }
}

impl From<Pos2D> for Point {
    fn from(value: Pos2D) -> Self {
        Self::new(value.x, value.y)
    }
}

impl From<Pos2D> for (i32, i32) {
    fn from(val: Pos2D) -> Self {
        (val.x, val.y)
    }
}
