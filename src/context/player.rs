use super::Pos3D;

pub struct Player {
    /// Player position in 3D space (how much everything will be moved in the opposite direction)
    pub pos: Pos3D,
    /// The horizontal turn angle (in degrees)
    pub turn: i32,
    /// Vertical look up/down
    pub look: i32,
}

impl Player {
    pub const fn new(pos: Pos3D) -> Self {
        Self {
            pos,
            turn: 0,
            look: 0,
        }
    }
}
