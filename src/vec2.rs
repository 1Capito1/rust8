use crate::config;
#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    // multiplies points by scale factor, effecitively

    pub fn create_point(x: f32, y: f32, conf: &config::Config) -> Self {
        Self {
            x: x * conf.scale_factor,
            y: y * conf.scale_factor,
        }
    }
}
