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
    let mut f = File::open("BC_test.ch8")?;
    let mut rom: Vec<u8> = Vec::new();
    let bytes_read = f.read_to_end(&mut rom)?;
    if bytes_read == 0 {
        panic!("Empty File");
    }
    request_new_screen_size(config.get_scaled_width(), config.get_scaled_height());
    // TEST
    cpu.modify_display();
    while !(is_key_pressed(KeyCode::Escape) || macroquad::input::is_quit_requested()) {
        clear_background(BLACK);
        draw(&cpu, &config);
        cpu.update_keypad_state();
        // Fetch instruction from memory at current PC
        let instruction = cpu.fetch_instruction(&rom);
        // Decode the instruction
        cpu.decode(instruction);

        // Execute instruction

        next_frame().await
    }
    Ok(())
}

fn draw(cpu: &cpu::Cpu, config: &config::Config) {
    for (i, _) in cpu.get_display().iter().enumerate() {
        for (j, _) in cpu.get_display()[i].iter().enumerate() {
            if cpu.get_display()[i][j] {
                let point = vec2::Vec2::create_point(i as f32, j as f32, config);
                draw_rectangle(point.x, point.y, config.scale_factor, config.scale_factor, WHITE);
            }
        }
    }
}
