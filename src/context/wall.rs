use sdl2::{pixels::Color, rect::Point};

use crate::renderer;

pub struct Wall {
    pub pos0: Point,
    pub pos1: Point,
    pub colour: Color,
}

impl Wall {
    pub const fn new(pos0: Point, pos1: Point, colour: Color) -> Self {
        Self { pos0, pos1, colour }
    }

    pub fn from_raw(x0: i32, y0: i32, x1: i32, y1: i32, colour_id: i32) -> Self {
        Self::new(
            Point::new(x0, y0),
            Point::new(x1, y1),
            renderer::colour_from_id(colour_id),
        )
    }
}