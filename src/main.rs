mod instruction;
mod config;
mod cpu;
mod vec2;
use macroquad::prelude::*;
use std::fs::File;
use std::io::Read;

#[macroquad::main("BasicShapes")]
async fn main() -> std::io::Result<()> {
    let config = config::Config::default();
    let mut cpu = cpu::Cpu::init();
    let mut f = File::open("src/roms/PONG")?;
    let mut rom: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut rom)?;
    cpu.load_rom(&rom).expect("File size 0");
    request_new_screen_size(config.get_scaled_width(), config.get_scaled_height());
    while !(is_key_pressed(KeyCode::Escape) || macroquad::input::is_quit_requested()) {
    clear_background(BLACK);
        cpu.update_keypad_state();
        let start = std::time::Instant::now();

        // Fetch instruction from memory at current PC
        for _ in 0..700/60 {
            let instruction = cpu.fetch_instruction();
            // Decode the instruction and execute
            cpu.decode(instruction, &config);
        }

        let elapsed = start.elapsed();
        let sleep_duration = if 16.67 > elapsed.as_millis() as f32 {16.67 - elapsed.as_millis() as f32 / 1000.0} else {0.0} as u64;
        std::thread::sleep(std::time::Duration::from_millis(sleep_duration));

        if cpu.will_draw() {
            cpu.draw(&config);
        }


        cpu.update_timers();
        next_frame().await
    }
    Ok(())
}

