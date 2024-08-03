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
    let mut f = File::open("IBMLogo.ch8")?;
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
            draw(&mut cpu, &config);
        }


        next_frame().await
    }
    Ok(())
}

fn draw(cpu: &mut cpu::Cpu, config: &config::Config) {
    // Get the display data once
    let display = cpu.get_display();

    // Iterate over display pixels
    for (i, row) in display.iter().enumerate() {
        for (j, &pixel_on) in row.iter().enumerate() {
            if pixel_on {
                // Create point for drawing
                let v = vec2::Vec2::create_point(j as f32, i as f32, config);

                // Draw rectangle representing the pixel
                draw_rectangle(v.y, v.x, config.scale_factor, config.scale_factor, WHITE);
            }
        }
    }
}
