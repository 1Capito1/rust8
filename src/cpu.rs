// non_snake_case allowed for I and V registers
use macroquad::prelude::*;
#[allow(non_snake_case)]
#[allow(dead_code)]
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
    draw: bool
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
            draw: false
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
    pub fn will_draw(&self) -> bool {
        return self.draw;
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
        println!("Bytes read: {:02x} {:02x}", bytes[0], bytes[1]);

        let shifted = (bytes[0] as u16) << 8;
        println!("Shifted: {:04x}", shifted);

        let result: u16 = shifted | (bytes[1] as u16);
        println!("Result: {:04x}", result);

        self.pc += 2;
        return result;
    }

    pub fn decode(&mut self, instr: u16, config: &crate::config::Config) {
        println!("{instr}");
        match (instr >> 12) & 0x0F {
            0x0 => match (instr >> 4) & 0x0FFF {
                // 0x0E0: clear screen
                0x0E0 => self.display = [[false; 32]; 64],
                _ => todo!(),
            },
            0x1 => {
                // 0x1NNN: jump to NNN
                let address = (instr >> 4) & 0xFFF;
                self.pc = address as usize;
            },
            0x6 => {
                // 0x6XNN: set VX to NN
                let vx: usize = (instr >> 8) as usize & 0xF;
                self.V[vx] = (instr & 0xFF) as u8;
            },
            0x7 => {
                // 0x7XNN: add NN to VX
                let vx: usize = (instr >> 8) as usize & 0xF;
                self.V[vx] += (instr & 0xFF) as u8;
            },
            0xA => {
                // ANNN: set I to the address of NNN
                let nnn = instr & 0xFFF;
                self.I = nnn;
            },
            0xD => {
                // DXYN: draw sprite at VX, VY, width 8 height N
                let x = (instr >> 8) as usize & 0xF;
                let y = (instr >> 4) as usize & 0xF;
                let vx = self.V[x];
                let vy = self.V[y];
                self.V[0xF] = 0;
                let mut pos_x = vx & 61;
                let mut pos_y = vy & 31;
                let n = instr & 0xF;
                for i in 0..n {
                    let sprite_data = self.memory[self.I as usize + i as usize];
                    for j in 7..=0 {
                        let pixel: &mut bool = &mut self.display[pos_y as usize * config.get_scaled_width() as usize][pos_x as usize];
                        let sprite_bit: bool = sprite_data & (1 << j) == 1;
                        if sprite_bit && *pixel {
                            self.V[0xF] = 1;
                        }

                        *pixel ^= sprite_bit;

                        pos_x += 1;

                        if pos_x as f32 >= config.get_scaled_width() {break;}
                    }
                    pos_y += 1;
                    if pos_y as f32 >= config.get_scaled_height() {break;}
                }
            }
            _ => todo!(),
        }
    }
}
