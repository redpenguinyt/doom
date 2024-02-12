use core::f64;

use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

use crate::{
    context::{Context, Rotation},
    types::{Pos2D, Pos3D},
};

mod colour;
mod rendered_sector;

pub use colour::colour_from_id;

use self::rendered_sector::RenderedSector;

pub const VSCREEN_WIDTH: u32 = 160;
pub const VSCREEN_HEIGHT: u32 = 120;
pub const PIXEL_SCALE: u32 = 8;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Self, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Self { canvas })
    }

    fn plot(&mut self, point: Pos2D) -> Result<(), String> {
        let flipped_point = Pos2D::new(point.x, VSCREEN_HEIGHT as i32 - 1 - point.y); // Make (0,0) the bottom left corner instead of top left
        self.canvas.fill_rect(Rect::new(
            flipped_point.x * PIXEL_SCALE as i32,
            flipped_point.y * PIXEL_SCALE as i32,
            PIXEL_SCALE,
            PIXEL_SCALE,
        ))?;

        Ok(())
    }

    fn plot_pixel(&mut self, point: Pos2D, color: Color) -> Result<(), String> {
        if Rect::new(0, 0, VSCREEN_WIDTH, VSCREEN_HEIGHT).contains_point(point) {
            self.canvas.set_draw_color(color);
            self.plot(point)?;
        }

        Ok(())
    }

    fn draw_background(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 60, 130));
        self.canvas.clear();

        Ok(())
    }

    pub fn draw(&mut self, context: &mut Context) -> Result<(), String> {
        // TODO: avoid mutating context
        self.canvas
            .window_mut()
            .set_size(VSCREEN_WIDTH * PIXEL_SCALE, VSCREEN_HEIGHT * PIXEL_SCALE)
            .map_err(|e| e.to_string())?;
        self.draw_background()?;

        self.draw_3d(context)?;

        self.canvas.present();

        Ok(())
    }

    pub fn draw_3d(&mut self, context: &mut Context) -> Result<(), String> {
        let mut w = [Pos3D::ZERO; 4];
        let player = &context.player;
        let rotation = Rotation::from_degrees(player.turn);

        context.sectors.sort_unstable_by_key(|s| -s.distance);

        for sector in &mut context.sectors {
            sector.distance = 0;

            let mut rsector = RenderedSector::new(sector, player.pos.z);

            for l in 0..=1 {
                for wall in &context.walls[sector.wall_index_start..sector.wall_index_end] {
                    let wall_pos0 = wall.pos0 - player.pos.into();
                    let wall_pos1 = wall.pos1 - player.pos.into();

                    // Swap for surface
                    let (wall_pos0, wall_pos1) = if l == 0 {
                        (wall_pos1, wall_pos0)
                    } else {
                        (wall_pos0, wall_pos1)
                    };

                    let (x0, y0): (f64, f64) = wall_pos0.into();
                    let (x1, y1): (f64, f64) = wall_pos1.into();

                    // World X position
                    w[0].x = x0.mul_add(rotation.cos, -(y0 * rotation.sin)) as i32;
                    w[1].x = x1.mul_add(rotation.cos, -(y1 * rotation.sin)) as i32;

                    // World Y position (depth)
                    w[0].y = y0.mul_add(rotation.cos, x0 * rotation.sin) as i32;
                    w[1].y = y1.mul_add(rotation.cos, x1 * rotation.sin) as i32;

                    // Add this wall's distance to the total distance (and average it out later)
                    sector.distance += (Pos2D::from(w[0] + w[1]) / 2).magnitude() as i32;

                    // World Z height
                    w[0].z = sector.z0 - player.pos.z + (player.look * w[0].y) / 32;
                    w[1].z = sector.z0 - player.pos.z + (player.look * w[1].y) / 32;

                    // Duplicate bottom positions to top positions (and adjust z)
                    w[2] = w[0];
                    w[3] = w[1];
                    w[2].z += sector.z1;
                    w[3].z += sector.z1;

                    // Don't draw if behind player
                    if w[0].y < 1 && w[1].y < 1 {
                        continue;
                    }
                    // Clip beginning of wall
                    if w[0].y < 1 {
                        w[0] = w[0].clip_behind_player(w[1]);
                        w[2] = w[2].clip_behind_player(w[3]);
                    }
                    // Clip end of wall
                    if w[1].y < 1 {
                        w[1] = w[1].clip_behind_player(w[0]);
                        w[3] = w[3].clip_behind_player(w[2]);
                    }

                    // Convert positions to screen (200 is the FOV)
                    let screen_positions = w.map(|pos| {
                        Pos2D::new(
                            pos.x * 200 / pos.y + (VSCREEN_WIDTH as i32 / 2),
                            pos.z * 200 / pos.y + (VSCREEN_HEIGHT as i32 / 2),
                        )
                    });

                    self.draw_wall(
                        screen_positions[0],
                        screen_positions[1],
                        screen_positions[2].y,
                        screen_positions[3].y,
                        wall.colour,
                        &mut rsector,
                    )?;
                }

                // Average out distance of walls to get the distance of the sector from the player
                sector.distance /= (sector.wall_index_end - sector.wall_index_start) as i32;
                // Flip to negative to draw top/bottom surface
                rsector.surface *= -1;
            }
        }

        Ok(())
    }

    fn draw_wall(
        &mut self,
        pos0: Pos2D,
        pos1: Pos2D,
        top_depth0: i32,
        top_depth1: i32,
        color: Color,
        sector: &mut RenderedSector,
    ) -> Result<(), String> {
        let dyb = f64::from(pos1.y - pos0.y);
        let dyt = f64::from(top_depth1 - top_depth0);
        let mut dx = f64::from(pos1.x - pos0.x);
        if dx == 0.0 {
            dx = 1.0;
        };
        let xs = f64::from(pos0.x);

        // Clip X
        let x0 = pos0.x.clamp(0, VSCREEN_WIDTH as i32);
        let x1 = pos1.x.clamp(0, VSCREEN_WIDTH as i32);

        for x in x0..x1 {
            let y0 = (dyb * (f64::from(x) - xs + 0.5) / dx + f64::from(pos0.y)).round() as i32;
            let y1 = (dyt * (f64::from(x) - xs + 0.5) / dx + f64::from(top_depth0)).round() as i32;

            let y0 = y0.clamp(0, VSCREEN_HEIGHT as i32);
            let y1 = y1.clamp(0, VSCREEN_HEIGHT as i32);

            // Top and bottom
            match sector.surface {
                1 => {
                    sector.surf[x as usize] = y0;
                    continue;
                }
                2 => {
                    sector.surf[x as usize] = y1;
                    continue;
                }

                -1 => {
                    for y in sector.surf[x as usize]..y0 {
                        self.plot_pixel(Pos2D::new(x, y), sector.bottom_colour)?;
                    }
                }
                -2 => {
                    for y in y1..sector.surf[x as usize] {
                        self.plot_pixel(Pos2D::new(x, y), sector.top_colour)?;
                    }
                }

                _ => {}
            }

            for y in y0..y1 {
                self.plot_pixel(Pos2D::new(x, y), color)?;
            }
        }

        Ok(())
    }
}
