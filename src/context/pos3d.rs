#[derive(Debug, Clone, Copy)]
pub struct Pos3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos3D {
    pub const ZERO: Self = Pos3D::new(0, 0, 0);

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}
