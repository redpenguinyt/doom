use std::{
    thread,
    time::{Duration, Instant},
};

const FPS: u32 = 20;
const TICK_RATE: u32 = 1;

pub struct GameTick {
    frame_started: Instant,
    pub tick: u32,
    pub tick_rate: u32,
}

impl GameTick {
    pub fn new() -> Self {
        Self {
            frame_started: Instant::now(),
            tick: 0,
            tick_rate: TICK_RATE,
        }
    }

    pub fn next_frame(&mut self) {
        self.tick += 1;
        self.tick %= self.tick_rate;
    }

    pub fn sleep_frame(&mut self) {
        if self.frame_started.elapsed() < Duration::from_secs(1) {
            thread::sleep(Duration::from_secs(1) / FPS - self.frame_started.elapsed());
        }
        self.frame_started = Instant::now()
    }
}
