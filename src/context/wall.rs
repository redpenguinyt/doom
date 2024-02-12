use sdl2::pixels::Color;

use crate::{renderer, types::Pos2D};

pub struct Wall {
    pub pos0: Pos2D,
    pub pos1: Pos2D,
    pub colour: Color,
}

impl Wall {
    pub const fn new(pos0: Pos2D, pos1: Pos2D, colour: Color) -> Self {
        Self { pos0, pos1, colour }
    }

    pub fn from_raw(x0: i32, y0: i32, x1: i32, y1: i32, colour_id: i32) -> Self {
        Self::new(
            Pos2D::new(x0, y0),
            Pos2D::new(x1, y1),
            renderer::colour_from_id(colour_id),
        )
    }

    pub fn many_from_raw(nums: Vec<i32>) -> Vec<Self> {
        let mut walls = vec![];

        for i in 0..nums.len() / 5 {
            let i = i * 5;
            walls.push(Self::from_raw(
                nums[i],
                nums[i + 1],
                nums[i + 2],
                nums[i + 3],
                nums[i + 4],
            ));
        }

        walls
    }
}
