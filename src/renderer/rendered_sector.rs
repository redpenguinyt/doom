use crate::{context::Sector, VSCREEN_WIDTH};
use sdl2::pixels::Color;

pub struct RenderedSector {
    pub top_colour: Color,
    pub bottom_colour: Color,
    pub surface: i32,
    pub surf: [i32; VSCREEN_WIDTH as usize],
}

impl RenderedSector {
    pub fn new(sector: &Sector, player_z: i32) -> Self {
        let surface = if player_z < sector.z0 {
            1 // Case 1: player is below sector (bottom is visible)
        } else if player_z > sector.z1 {
            2 // Case 1: player is above sector (top is visible)
        } else {
            0 // Player can see neither top nor bottom
        };

        Self {
            top_colour: sector.top_colour,
            bottom_colour: sector.bottom_colour,
            surface,
            surf: [0; VSCREEN_WIDTH as usize],
        }
    }
}
