use sdl2::{pixels::Color, rect::{Point, Rect}, render::WindowCanvas, video::Window};

use crate::context::GameContext;

pub const GRID_X_SIZE: u32 = 160;
pub const GRID_Y_SIZE: u32 = 120;
pub const DOT_SIZE_IN_PXS: u32 = 8;

pub struct Renderer {
	canvas: WindowCanvas
}

impl Renderer {
	pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Renderer { canvas })
    }

	fn plot(&mut self, point: Point) -> Result<(), String> {
        self.canvas.fill_rect(Rect::new(
            point.x * DOT_SIZE_IN_PXS as i32,
            point.y * DOT_SIZE_IN_PXS as i32,
            DOT_SIZE_IN_PXS,
            DOT_SIZE_IN_PXS,
        ))?;

        Ok(())
    }

	fn plot_pixel(&mut self, point: Point, color: Color) -> Result<(), String> {
		self.canvas.set_draw_color(color);
		self.plot(point)
	}

	fn draw_background(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        Ok(())
    }

	pub fn draw(&mut self, context: &GameContext) -> Result<(), String> {
		self.draw_background()?;

		self.plot_pixel(context.pixel_pos, Color::RED)?;

        self.canvas.present();

		Ok(())
	}
}