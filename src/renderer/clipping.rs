use crate::context::Pos3D;

pub fn clip_behind_player(pos0: Pos3D, pos1: Pos3D) -> Pos3D {
    let da = pos0.y as f64;
    let db = pos1.y as f64;

    let mut d = da - db;
    if d == 0.0 {
        d = 1.0;
    }
    let s = da / d;

    let mut p = Pos3D::new(
        (pos0.x as f64 + s * (pos1.x - pos0.x) as f64) as i32,
        (pos0.y as f64 + s * (pos1.y - pos0.y) as f64) as i32,
        (pos0.z as f64 + s * (pos1.z - pos0.z) as f64) as i32,
    );
    if p.y == 0 {
        // Prevent clipping
        p.y = 1
    };

    p
}
