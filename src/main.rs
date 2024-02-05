use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};

mod context;
use context::{GameContext, Rotation};
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
            VSCREEN_WIDTH * PIXEL_SCALE,
            VSCREEN_HEIGHT * PIXEL_SCALE,
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
                let player_rotation = Rotation::from_degrees(context.player.turn);
                let dx = (player_rotation.sin * 10.0) as i32;
                let dy = (player_rotation.cos * 10.0) as i32;

                match (m_pressed, scancode) {
                    // Without M
                    (false, Scancode::W) => {
                        // Move forward
                        context.player.pos.x += dx;
                        context.player.pos.y += dy;
                    }
                    (false, Scancode::S) => {
                        // Move backwards
                        context.player.pos.x -= dx;
                        context.player.pos.y -= dy;
                    }
                    // Look left
                    (false, Scancode::A) => context.player.turn -= 4,
                    // Look right
                    (false, Scancode::D) => context.player.turn += 4,

                    // With M
                    (true, Scancode::W) => {
                        // Move up
                        context.player.pos.z -= 4;
                    }
                    (true, Scancode::S) => {
                        // Move down
                        context.player.pos.z += 4;
                    }
                    (true, Scancode::A) => {
                        // Look up
                        context.player.look -= 1;
                    }
                    (true, Scancode::D) => {
                        // Look down
                        context.player.look += 1;
                    }

                    (_, Scancode::Period) => {
                        // Strafe left
                        context.player.pos.x += dy;
                        context.player.pos.y -= dx;
                    }
                    (_, Scancode::Comma) => {
                        // Strafe right
                        context.player.pos.x -= dy;
                        context.player.pos.y += dx;
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

        renderer.draw(&mut context)?;

        game_tick.sleep_frame();
    }

    Ok(())
}
