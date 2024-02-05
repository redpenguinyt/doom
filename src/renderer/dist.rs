use sdl2::rect::Point;

pub fn distance(pos0: Point, pos1: Point) -> i32 {
    (((pos1.x - pos0.x).pow(2) + (pos1.y - pos0.y).pow(2)) as f64)
        .sqrt()
        .round() as i32
}
