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
    let mut f = File::open("src/roms/test_opcode.ch8")?;
    let mut rom: Vec<u8> = Vec::new();
    let _ = f.read_to_end(&mut rom)?;
    cpu.load_rom(&rom).expect("File size 0");
    request_new_screen_size(config.get_scaled_width(), config.get_scaled_height());
    clear_background(BLACK);
    while !(is_key_pressed(KeyCode::Escape) || macroquad::input::is_quit_requested()) {
        cpu.update_keypad_state();
        // Fetch instruction from memory at current PC
        let instruction = cpu.fetch_instruction();
        // Decode the instruction and execute
        cpu.decode(instruction, &config);

        if cpu.will_draw() {
            cpu.draw(&config);
        }


        next_frame().await
    }
    Ok(())
}

