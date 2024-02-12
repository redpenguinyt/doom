mod player;
mod rotation;
mod sector;
mod wall;

use player::Player;
pub use rotation::Rotation;
use sdl2::keyboard::{KeyboardState, Scancode};
pub use sector::Sector;
pub use wall::Wall;

use crate::types::Pos3D;

pub struct Context {
    pub player: Player,
    pub walls: Vec<Wall>,
    pub sectors: Vec<Sector>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            player: Player::new(Pos3D::new(70, -110, 20)),
            walls: Wall::many_from_raw(vec![
                160,228, 168,228, 4,
                168,228, 176,236, 5,
                176,236, 176,244, 4,
                176,244, 168,252, 5,
                168,252, 160,252, 4,
                160,252, 152,244, 5,
                152,244, 152,236, 4,
                152,236, 160,228, 5,
                104,224, 152,184, 1,
                152,184, 176,184, 3,
                176,184, 224,224, 1,
                224,224, 224,256, 0,
                224,256, 192,288, 1,
                192,288, 136,288, 0,
                136,288, 104,256, 1,
                104,256, 104,224, 0,
                104,224, 152,184, 1,
                152,184, 176,184, 0,
                176,184, 224,224, 1,
                224,224, 224,256, 0,
                224,256, 192,288, 1,
                192,288, 136,288, 0,
                136,288, 104,256, 1,
                104,256, 104,224, 0,
                152,168, 176,168, 2,
                176,168, 176,184, 3,
                176,184, 152,184, 2,
                152,184, 152,168, 3,
                152,152, 176,152, 2,
                176,152, 176,168, 3,
                176,168, 152,168, 2,
                152,168, 152,152, 3,
                152,136, 176,136, 2,
                176,136, 176,152, 3,
                176,152, 152,152, 2,
                152,152, 152,136, 3,
                208,160, 208,136, 5,
                208,136, 232,136, 4,
                232,136, 232,160, 5,
                232,160, 208,160, 4,
                96, 136, 120,136, 4,
                120,136, 120,160, 5,
                120,160, 96, 160, 4,
                96, 160, 96, 136, 5,
                216,144, 224,144, 4,
                224,144, 224,152, 5,
                224,152, 216,152, 4,
                216,152, 216,144, 5,
                104,144, 112,144, 4,
                112,144, 112,152, 5,
                112,152, 104,152, 4,
                104,152, 104,144, 5,
            ]),
            sectors: Sector::many_from_raw(vec![
                0,8,   40,20,  9,9,
                8,16,  0,40,   6,6,
                16,24, 80,110, 0,0,
                24,28, 0,30,   6,6,
                28,32, 0,20,   6,6,
                32,36, 0,10,   6,6,
                36,40, 0,30,   5,5,
                40,44, 0,30,   5,5,
                44,48, 30,110, 0,0,
                48,52, 30,110, 0,0,
            ]),
        }
    }

    pub fn move_player(&mut self, keyboard_state: KeyboardState) {
        let m_pressed = keyboard_state.is_scancode_pressed(Scancode::M);
        for (scancode, pressed) in keyboard_state.scancodes() {
            if pressed {
                let player_rotation = Rotation::from_degrees(self.player.turn);
                let dx = (player_rotation.sin * 10.0) as i32;
                let dy = (player_rotation.cos * 10.0) as i32;

                match (m_pressed, scancode) {
                    // Without M
                    (false, Scancode::W) => {
                        // Move forward
                        self.player.pos.x += dx;
                        self.player.pos.y += dy;
                    }
                    (false, Scancode::S) => {
                        // Move backwards
                        self.player.pos.x -= dx;
                        self.player.pos.y -= dy;
                    }
                    // Look left
                    (false, Scancode::A) => self.player.turn -= 4,
                    // Look right
                    (false, Scancode::D) => self.player.turn += 4,

                    // With M
                    (true, Scancode::W) => {
                        // Move up
                        self.player.pos.z -= 4;
                    }
                    (true, Scancode::S) => {
                        // Move down
                        self.player.pos.z += 4;
                    }
                    (true, Scancode::A) => {
                        // Look up
                        self.player.look -= 1;
                    }
                    (true, Scancode::D) => {
                        // Look down
                        self.player.look += 1;
                    }

                    (_, Scancode::Period) => {
                        // Strafe left
                        self.player.pos.x += dy;
                        self.player.pos.y -= dx;
                    }
                    (_, Scancode::Comma) => {
                        // Strafe right
                        self.player.pos.x -= dy;
                        self.player.pos.y += dx;
                    }

                    (_, _) => (),
                }
            }
        }
    }
}
