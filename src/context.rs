use sdl2::rect::Point;

pub struct GameContext {
    pub pixel_pos: Point,
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            pixel_pos: Point::new(10, 10),
        }
    }

    pub fn tick(&mut self) {
        self.pixel_pos.y += 1;

        if self.pixel_pos.y > 30 {
            self.pixel_pos.y = 10;
        }
    }
}
