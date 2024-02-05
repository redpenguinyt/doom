use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

mod context;
use context::GameContext;
mod renderer;
use renderer::*;
mod tick;
use tick::GameTick;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "Snake",
            GRID_X_SIZE * DOT_SIZE_IN_PXS,
            GRID_Y_SIZE * DOT_SIZE_IN_PXS,
        )
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut renderer = Renderer::new(window)?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut context = GameContext::new();
    let mut game_tick = GameTick::new();

    'running: loop {
        let m_pressed = event_pump.keyboard_state().is_scancode_pressed(Scancode::M);
        for (scancode, pressed) in event_pump.keyboard_state().scancodes() {
            if pressed {
                match (m_pressed, scancode) {
                    (false, Scancode::W) => {
                        println!("up")
                    }
                    (true, Scancode::W) => {
                        println!("move up")
                    }
                    (false, Scancode::A) => {
                        println!("left")
                    }
                    (true, Scancode::A) => {
                        println!("look up")
                    }
                    (false, Scancode::S) => {
                        println!("down")
                    }
                    (true, Scancode::S) => {
                        println!("move down")
                    }
                    (false, Scancode::D) => {
                        println!("right")
                    }
                    (true, Scancode::D) => {
                        println!("look down")
                    }
                    (_, Scancode::Period) => {
                        // < key
                        println!("strafe right")
                    }
                    (_, Scancode::Comma) => {
                        // > key
                        println!("strafe left")
                    }

                    (_, _) => (),
                }
            }
        }

        for event in event_pump.poll_iter() {
            #[allow(clippy::single_match)]
            match event {
                Event::Quit { .. } => break 'running,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape => break 'running,
                    Keycode::Y => {
                        println!("yoohoo!")
                    }

                    _ => (),
                },

                _ => (),
            }
        }

        game_tick.next_frame();
        if game_tick.tick == 0 {
            context.tick();
        };

        renderer.draw(&context)?;

        game_tick.sleep_frame();
    }

    Ok(())
}
