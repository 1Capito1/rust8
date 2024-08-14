#![allow(dead_code)]
use std::fs::File;
const SCREEN_WIDTH: f32 = 64.0;
const SCREEN_HEIGHT: f32 = 32.0;
const SCALE_FACTOR: f32 = 25.0;

pub struct Config {
    // Screen config
    screen_height: f32,
    screen_width: f32,
    pub scale_factor: f32,

    // Emulator config
    pub file_path: Result<File, std::io::Error>,
    pub instr_per_second: u32,
    pub legacy: bool,
    pub bxnn_quirk: bool,
    pub frame_rate: f32,
}

impl Config {
    pub fn get_scaled_height(&self) -> f32 {
        return self.screen_height * self.scale_factor;
    }
    pub fn get_scaled_width(&self) -> f32 {
        return self.screen_width * self.scale_factor;
    }
    pub fn get_raw_width(&self) -> f32 {
        return self.screen_width;
    }
    pub fn get_raw_height(&self) -> f32 {
        return self.screen_height;
    }
}

/// Default Values:
///     screen_height: 32
///     screen_width: 64
///     scale_factor: 20
///     file_path: required arg and therefore panics
impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            screen_height: SCREEN_HEIGHT,
            screen_width: SCREEN_WIDTH,
            scale_factor: SCALE_FACTOR,
            file_path: File::open("Unknown Path"),
            instr_per_second: 0,
            legacy: false,
            bxnn_quirk: false,
            frame_rate: 60.0,
        }
    }
}
