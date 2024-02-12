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

    pub fn magnitude(self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt().round()
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
        (f64::from(val.x), f64::from(val.y))
    }
}

impl From<Pos2D> for (i32, i32) {
    fn from(val: Pos2D) -> Self {
        (val.x, val.y)
    }
}

impl Add<Self> for Pos2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Self> for Pos2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Div<i32> for Pos2D {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
