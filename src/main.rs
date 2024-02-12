use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod context;
mod renderer;
mod tick;
mod types;

use context::Context;
use renderer::{colour_from_id, Renderer, PIXEL_SCALE, VSCREEN_HEIGHT, VSCREEN_WIDTH};
use tick::GameTick;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Doom but good",
            VSCREEN_WIDTH * PIXEL_SCALE,
            VSCREEN_HEIGHT * PIXEL_SCALE,
        )
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut context = Context::new();
    let mut game_tick = GameTick::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => (),
            }
        }

        context.move_player(event_pump.keyboard_state());

        game_tick.next_frame();
        if game_tick.tick == 0 {
            // context.tick();
        };

        renderer.draw(&mut context)?;

        game_tick.sleep_frame();
    }

    Ok(())
}
