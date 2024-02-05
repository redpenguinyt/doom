use std::ops::{Add, Div, Sub};

use sdl2::rect::Point;

use super::Pos3D;

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

impl From<Pos3D> for Pos2D {
    fn from(val: Pos3D) -> Self {
        Self::new(val.x, val.y)
    }
}

impl From<Pos2D> for Point {
    fn from(val: Pos2D) -> Self {
        Self::new(val.x, val.y)
    }
}

impl From<Pos2D> for (f64, f64) {
    fn from(val: Pos2D) -> Self {
        (val.x as f64, val.y as f64)
    }
}

impl From<Pos2D> for (i32, i32) {
    fn from(val: Pos2D) -> Self {
        (val.x, val.y)
    }
}

impl Add<Pos2D> for Pos2D {
    type Output = Pos2D;

    fn add(self, rhs: Pos2D) -> Self::Output {
        Pos2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Pos2D> for Pos2D {
    type Output = Pos2D;

    fn sub(self, rhs: Pos2D) -> Self::Output {
        Pos2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Div<i32> for Pos2D {
    type Output = Pos2D;

    fn div(self, rhs: i32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
