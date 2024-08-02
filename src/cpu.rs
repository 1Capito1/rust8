// non_snake_case allowed for I and V registers
#[allow(non_snake_case)]
use macroquad::prelude::*;
pub struct Cpu {
    memory: [u8; 4096],
    pc: usize,
    I: u16,
    stack: [u16; 12],
    delay_timer: u8,
    sound_timer: u8,
    V: [u8; 16],
    keypad: [bool; 16],
    display: [[bool; 32]; 64],
}

impl Cpu {
    fn new() -> Self {
        Self {
            memory: [0; 4096],
            pc: 0,
            I: 0,
            stack: [0; 12],
            delay_timer: 0,
            sound_timer: 0,
            V: [0; 16],
            keypad: [false; 16],
            display: [[false; 32]; 64],
        }
    }

    pub fn init() -> Self {
        let mut cpu = Self::new();

        let font: &[u8] = &[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];

        let mut j = 0;
        for i in 0x050..=0x09F {
            cpu.memory[i] = font[j];
            j += 1;
        }

        return cpu;
    }
    pub fn get_display(&self) -> &[[bool; 32]; 64] {
        return &self.display;
    }
    // REMOVE
    pub fn modify_display(&mut self) {
        for i in 0..self.display.len() {
            for j in 0..self.display[i].len() {
                if (i + j) % 2 == 0 {
                    self.display[i][j] = true;
                }
            }
        }
    }
    pub fn update_keypad_state(&mut self) {
        use macroquad::input::KeyCode as KeyCode;
        use macroquad::input::is_key_down as is_key_down;
        self.keypad = [false; 16];
        if is_key_down(KeyCode::Key1) {
            self.keypad[0] = true;
        }
        if is_key_down(KeyCode::Key2) {
            self.keypad[1] = true;
        }
        if is_key_down(KeyCode::Key3) {
            self.keypad[2] = true;
        }
        if is_key_down(KeyCode::Key4) {
            self.keypad[3] = true;
        }
        if is_key_down(KeyCode::Q) {
            self.keypad[4] = true;
        }
        if is_key_down(KeyCode::W) {
            self.keypad[5] = true;
        }
        if is_key_down(KeyCode::E) {
            self.keypad[6] = true;
        }
        if is_key_down(KeyCode::R) {
            self.keypad[7] = true;
        }
        if is_key_down(KeyCode::A) {
            self.keypad[8] = true;
        }
        if is_key_down(KeyCode::S) {
            self.keypad[9] = true;
        }
        if is_key_down(KeyCode::D) {
            self.keypad[10] = true;
        }
        if is_key_down(KeyCode::F) {
            self.keypad[11] = true;
        }
        if is_key_down(KeyCode::Z) {
            self.keypad[12] = true;
        }
        if is_key_down(KeyCode::X) {
            self.keypad[13] = true;
        }
        if is_key_down(KeyCode::C) {
            self.keypad[14] = true;
        }
        if is_key_down(KeyCode::V) {
            self.keypad[15] = true;
        }
    }

    pub fn fetch_instruction(&mut self, rom: &[u8]) -> u16 {
        let bytes: [u8; 2] = [rom[self.pc], rom[self.pc + 1]];
        let result: u16 = ((bytes[0] as u16) << 8) | (bytes[1] as u16);
        self.pc += 2;
        return result;
    }

    pub fn decode(&mut self, instr: u16) {
        match (instr >> 12) & 0xF {
            0x0 => match (instr >> 4) & 0x0FFF {
                0x0E0 => scene::clear(),
                _ => todo!(),
            }
            _ => todo!(),
        }
    }
}
