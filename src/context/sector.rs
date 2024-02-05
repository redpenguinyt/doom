use sdl2::{pixels::Color, rect::Point};

use crate::{colour_from_id, VSCREEN_WIDTH};

pub struct Sector {
    pub wall_index_start: usize,
    pub wall_index_end: usize,
    /// Wall bottom height
    pub z0: i32,
    /// Wall top height
    pub z1: i32,
    /// Centre position for sector
    pub pos: Point,
    /// add y distances to sort drawing order
    pub distance: i32,

	pub top_colour: Color,
    pub bottom_colour: Color,

	pub surf: [i32; VSCREEN_WIDTH as usize],
	pub surface: i32
}

impl Sector {
    pub fn new(
        wall_index_start: usize,
        wall_index_end: usize,
        z0: i32,
        z1: i32,
        top_colour: Color,
        bottom_colour: Color,
    ) -> Self {
        Self {
            wall_index_start,
            wall_index_end,
            z0,
            z1,
            pos: Point::new(0, 0),
            distance: 0,
            top_colour,
            bottom_colour,
			surf: [0; VSCREEN_WIDTH as usize],
			surface: 0
        }
    }

    pub fn from_raw(
        wall_index_start: usize,
        wall_index_end: usize,
        z0: i32,
        z1: i32,
        top_colour_id: i32,
        bottom_colour_id: i32,
    ) -> Self {
        Self::new(
            wall_index_start,
            wall_index_end,
            z0,
            z1,
            colour_from_id(top_colour_id),
            colour_from_id(bottom_colour_id),
        )
    }
}
