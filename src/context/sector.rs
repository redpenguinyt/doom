use sdl2::pixels::Color;

use crate::{colour_from_id, types::Pos2D};

pub struct Sector {
    pub wall_index_start: usize,
    pub wall_index_end: usize,
    /// Wall bottom height
    pub z0: i32,
    /// Wall top height (relative to bottom)
    pub z1: i32,
    /// Centre position for sector
    pub pos: Pos2D,
    /// add y distances to sort drawing order
    pub distance: i32,

    pub top_colour: Color,
    pub bottom_colour: Color,
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
            pos: Pos2D::new(0, 0),
            distance: 0,
            top_colour,
            bottom_colour,
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

    pub fn many_from_raw(nums: Vec<i32>) -> Vec<Self> {
        let mut walls = vec![];

        for i in 0..nums.len() / 6 {
            let i = i * 6;
            walls.push(Self::from_raw(
                nums[i] as usize,
                nums[i + 1] as usize,
                nums[i + 2],
                nums[i + 3],
                nums[i + 4],
                nums[i + 5],
            ));
        }

        walls
    }
}
