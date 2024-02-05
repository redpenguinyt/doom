use sdl2::{pixels::Color, rect::Rect, render::WindowCanvas, video::Window};

use crate::{
    context::{GameContext, Rotation, Sector},
    types::{Pos2D, Pos3D},
};

mod colour;

pub use colour::colour_from_id;

pub const VSCREEN_WIDTH: u32 = 160 * 8;
pub const VSCREEN_HEIGHT: u32 = 120 * 8;
pub const PIXEL_SCALE: u32 = 1;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Renderer { canvas })
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

    pub fn draw(&mut self, context: &mut GameContext) -> Result<(), String> {
        // TODO: avoid mutating context
        self.draw_background()?;

        self.draw_3d(context)?;

        self.canvas.present();

        Ok(())
    }

    pub fn draw_3d(&mut self, context: &mut GameContext) -> Result<(), String> {
        let mut w = [Pos3D::ZERO; 4];
        let player = &context.player;
        let rotation = Rotation::from_degrees(player.turn);

        context.sectors.sort_unstable_by_key(|s| -s.distance);

        for sector in &mut context.sectors {
            sector.distance = 0;

            let mut surface = if player.pos.z < sector.z0 {
                1
            } else if player.pos.z > sector.z1 {
                2
            } else {
                0
            };
            let mut surf = [0; VSCREEN_WIDTH as usize];

            for l in 0..=1 {
                for wall in &context.walls[sector.wall_index_start..sector.wall_index_end] {
                    let x0 = (wall.pos0.x - player.pos.x) as f64;
                    let y0 = (wall.pos0.y - player.pos.y) as f64;
                    let x1 = (wall.pos1.x - player.pos.x) as f64;
                    let y1 = (wall.pos1.y - player.pos.y) as f64;

                    // Swap for surface
                    let (x0, y0, x1, y1) = if l == 0 {
                        (x1, y1, x0, y0)
                    } else {
                        (x0, y0, x1, y1)
                    };

                    // World X position
                    w[0].x = (x0 * rotation.cos - y0 * rotation.sin) as i32;
                    w[1].x = (x1 * rotation.cos - y1 * rotation.sin) as i32;
                    w[2].x = w[0].x;
                    w[3].x = w[1].x;

                    // World Y position (depth)
                    w[0].y = (y0 * rotation.cos + x0 * rotation.sin) as i32;
                    w[1].y = (y1 * rotation.cos + x1 * rotation.sin) as i32;
                    w[2].y = w[0].y;
                    w[3].y = w[1].y;

                    // Store this wall's distance
                    sector.distance +=
                        Pos2D::new((w[0].x + w[1].x) / 2, (w[0].y + w[1].y) / 2).magnitude() as i32;

                    // World Z height
                    w[0].z =
                        sector.z0 - player.pos.z + ((player.look * w[0].y) as f64 / 32.0) as i32;
                    w[1].z =
                        sector.z0 - player.pos.z + ((player.look * w[1].y) as f64 / 32.0) as i32;
                    w[2].z = w[0].z + sector.z1;
                    w[3].z = w[1].z + sector.z1;

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
                    for pos in w.as_mut() {
                        pos.x = pos.x * 200 / pos.y + (VSCREEN_WIDTH as i32 / 2);
                        pos.y = pos.z * 200 / pos.y + (VSCREEN_HEIGHT as i32 / 2);
                    }

                    self.draw_wall(
                        w[0].x,
                        w[1].x,
                        w[0].y,
                        w[1].y,
                        w[2].y,
                        w[3].y,
                        wall.colour,
                        sector,
                        surface,
                        &mut surf,
                    )?;
                }

                // Average out distance of walls to get the distance of the sector from the player
                sector.distance /= (sector.wall_index_end - sector.wall_index_start) as i32;
                // Flip to negative to draw top/bottom surface
                surface *= -1;
            }
        }

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn draw_wall(
        &mut self,
        x0: i32,
        x1: i32,
        b0: i32,
        b1: i32,
        t0: i32,
        t1: i32,
        color: Color,
        sector: &Sector,
        surface: i32,
        surf: &mut [i32; VSCREEN_WIDTH as usize],
    ) -> Result<(), String> {
        let dyb = (b1 - b0) as f64;
        let dyt = (t1 - t0) as f64;
        let mut dx = (x1 - x0) as f64;
        if dx == 0.0 {
            dx = 1.0;
        };
        let xs = x0 as f64;

        // Clip X
        let x0 = x0.clamp(0, VSCREEN_WIDTH as i32);
        let x1 = x1.clamp(0, VSCREEN_WIDTH as i32);

        for x in x0..x1 {
            let y0 = (dyb * (x as f64 - xs + 0.5) / dx + b0 as f64) as i32;
            let y1 = (dyt * (x as f64 - xs + 0.5) / dx + t0 as f64) as i32;

            let y0 = y0.clamp(0, VSCREEN_HEIGHT as i32);
            let y1 = y1.clamp(0, VSCREEN_HEIGHT as i32);

            // Top and bottom
            match surface {
                1 => {
                    surf[x as usize] = y0;
                    continue;
                }
                2 => {
                    surf[x as usize] = y1;
                    continue;
                }

                -1 => {
                    for y in surf[x as usize]..y0 {
                        self.plot_pixel(Pos2D::new(x, y), sector.bottom_colour)?;
                    }
                }
                -2 => {
                    for y in y1..surf[x as usize] {
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
