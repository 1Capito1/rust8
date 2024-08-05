#![allow(dead_code)]
const SCREEN_WIDTH: f32 = 64.0;
const SCREEN_HEIGHT: f32 = 32.0;
const SCALE_FACTOR: f32 = 25.0;

pub struct Config {
    screen_height: f32,
    screen_width: f32,
    pub scale_factor: f32,
}

impl Config {
    pub fn create(screen_height: f32, screen_width: f32, scale_factor: f32) -> Self {
        Self {
            screen_height,
            screen_width,
            scale_factor,
        }
    }

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
impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            screen_height: SCREEN_HEIGHT,
            screen_width: SCREEN_WIDTH,
            scale_factor: SCALE_FACTOR,
        }
    }
}
