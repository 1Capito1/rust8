mod instruction;
mod opcodes;
mod config;
mod cpu;
mod vec2;
mod args;
use macroquad::prelude::*;
use std::error::Error;
use std::io::Read;

#[macroquad::main("BasicShapes")]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut config = config::Config::default();
    if let Err(e) = args::pass_args(&mut config) {
        eprintln!("{e}");
        return Err(Box::new(e));
    }

    let mut cpu = cpu::Cpu::init();
    let mut rom: Vec<u8> = Vec::new();
    let _ = config.file_path
        .as_ref()
        .unwrap()
        .read_to_end(&mut rom)?;
    cpu.load_rom(&rom).expect("File size 0");

    request_new_screen_size(config.get_scaled_width(), config.get_scaled_height());
    while !(is_key_pressed(KeyCode::Escape) || macroquad::input::is_quit_requested()) {  
        clear_background(BLACK);
        cpu.update_keypad_state();

        let start = std::time::Instant::now();
        // Fetch instruction from memory at current PC
        for _ in 0..config.instr_per_second {
            let instruction = cpu.fetch_instruction();
            // Decode the instruction and execute
            cpu.decode(instruction, &config);
        }
        let elapsed = start.elapsed();

        let hz = hz_from_millis(config.frame_rate);

        let sleep_duration = if hz > elapsed.as_millis() as f32 {hz - elapsed.as_millis() as f32 / 1000.0} else {0.0} as u64;
        std::thread::sleep(std::time::Duration::from_millis(sleep_duration));

        if cpu.will_draw() {
            cpu.draw(&config);
        }


        cpu.update_timers();
        next_frame().await
    }
    Ok(())
}

fn hz_from_millis(hz: f32) -> f32 {
    1.0 / hz * 1000.0
}

