use sdl2::pixels::Color;

pub fn colour_from_id(id: i32) -> Color {
    match id {
        0 => Color::RGB(255, 255, 0), // Yellow
        1 => Color::RGB(160, 160, 0), // Darker Yellow
        2 => Color::RGB(0, 255, 0),   // Green
        3 => Color::RGB(0, 160, 0),   // Darker Green
        4 => Color::RGB(0, 255, 255), // Cyan
        5 => Color::RGB(0, 160, 160), // Darker Cyan
        6 => Color::RGB(255, 0, 0),   // Red
        7 => Color::RGB(160, 0, 0),   // Darker Red
        8 => Color::RGB(0, 60, 130),  // Background

        _ => Color::MAGENTA, // No valid ID
    }
}
