mod config;
mod cpu;
mod vec2;
use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let config = config::Config::default();
    let mut cpu = cpu::Cpu::init();
    request_new_screen_size(config.get_scaled_width(), config.get_scaled_height());
    loop {
        clear_background(BLACK);
        if is_key_pressed(KeyCode::Escape) {
            return;
        }
        cpu.update_keypad_state();
        
        

        next_frame().await
    }
}
