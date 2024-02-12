pub struct Rotation {
    pub cos: f64,
    pub sin: f64,
}

impl Rotation {
    pub fn from_degrees(angle: i32) -> Self {
        let as_rad = f64::from(angle).to_radians();

        Self {
            cos: as_rad.cos(),
            sin: as_rad.sin(),
        }
    }
}
