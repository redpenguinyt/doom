mod player;
mod pos3d;
mod rotation;
mod wall;
mod sector;

use player::Player;
pub use pos3d::Pos3D;
pub use rotation::Rotation;
pub use wall::Wall;
pub use sector::Sector;

pub struct GameContext {
    pub player: Player,
    pub walls: Vec<Wall>,
    pub sectors: Vec<Sector>,
}

impl GameContext {
    pub fn new() -> Self {
        Self {
            player: Player::new(Pos3D::new(70, -110, 20)),
            walls: Wall::many_from_raw(vec![
                0, 0, 32, 0, 0,
                32, 0, 32, 32, 1,
                32, 32, 0, 32, 0,
                0, 32, 0, 0, 1,
                64, 0, 96, 0, 2,
                96, 0, 96, 32, 3,
                96, 32, 64, 32, 2,
                64, 32, 64, 0, 3,
                64, 64, 96, 64, 4,
                96, 64, 96, 96, 5,
                96, 96, 64, 96, 4,
                64, 96, 64, 64, 5,
                0, 64, 32, 64, 6,
                32, 64, 32, 96, 7,
                32, 96, 0, 96, 6,
                0, 96, 0, 64, 7,
            ]),
            sectors: Sector::many_from_raw(vec![
                0, 4, 0, 40, 2, 3,
                4, 8, 0, 40, 4, 5,
                8, 12, 0, 40, 6, 7,
                12, 16, 0, 40, 0, 1,
            ]),
        }
    }

    pub fn tick(&mut self) {}
}
