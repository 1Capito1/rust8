// non_snake_case allowed for I and V registers
use macroquad::prelude::*;
use crate::vec2::Vec2;
use crate::instruction::Instruction;

#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Cpu {
    memory: [u8; 4096],
    pc: usize,
    I: u16,
    stack: Vec<u16>,
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
            pc: 0x200,
            I: 0,
            stack: Vec::with_capacity(12),
            delay_timer: 0,
            sound_timer: 0,
            V: [0; 16],
            keypad: [false; 16],
            display: [[false; 32]; 64],
            draw: false
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 { self.delay_timer -= 1 }
        if self.sound_timer > 0 { self.sound_timer -= 1 }
    }

    pub fn get_keypad(&mut self) -> &mut [bool; 16] {
        return &mut self.keypad;
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
        cpu.memory[0x050..=0x09F].copy_from_slice(font);

        return cpu;
    }
    pub fn get_display(&self) -> &[[bool; 32]; 64] {
        return &self.display;
    }
    pub fn will_draw(&self) -> bool {
        return self.draw;
    }
    pub fn set_draw(&mut self, b: bool) {
        self.draw = b;
    }
    pub fn update_keypad_state(&mut self) {
        use macroquad::input::KeyCode as KeyCode;
        use macroquad::input::is_key_released as is_key_released;
        // 1 2 3 4      1 2 3 C
        // Q W E R      4 5 6 D
        // A S D F      7 8 9 E
        // Z X C V      A 0 B F
        self.keypad = [false; 16];
        if is_key_down(KeyCode::Key1) {
            self.keypad[1] = true;
        }
        if is_key_down(KeyCode::Key2) {
            self.keypad[2] = true;
        }
        if is_key_down(KeyCode::Key3) {
            self.keypad[3] = true;
        }
        if is_key_down(KeyCode::Key4) {
            self.keypad[0xC] = true;
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
            self.keypad[0xD] = true;
        }

        if is_key_down(KeyCode::A) {
            self.keypad[7] = true;
        }
        if is_key_down(KeyCode::S) {
            self.keypad[8] = true;
        }
        if is_key_down(KeyCode::D) {
            self.keypad[9] = true;
        }
        if is_key_down(KeyCode::F) {
            self.keypad[0xE] = true;
        }

        if is_key_down(KeyCode::Z) {
            self.keypad[0xA] = true;
        }
        if is_key_down(KeyCode::X) {
            self.keypad[0] = true;
        }
        if is_key_down(KeyCode::C) {
            self.keypad[0xB] = true;
        }
        if is_key_down(KeyCode::V) {
            self.keypad[0xF] = true;
        }

        // released
        if is_key_released(KeyCode::Key1) {
            self.keypad[1] = false;
        }
        if is_key_released(KeyCode::Key2) {
            self.keypad[2] = false;
        }
        if is_key_released(KeyCode::Key3) {
            self.keypad[3] = false;
        }
        if is_key_released(KeyCode::Key4) {
            self.keypad[0xC] = false;
        }

        if is_key_released(KeyCode::Q) {
            self.keypad[4] = false;
        }
        if is_key_released(KeyCode::W) {
            self.keypad[5] = false;
        }
        if is_key_released(KeyCode::E) {
            self.keypad[6] = false;
        }
        if is_key_released(KeyCode::R) {
            self.keypad[0xD] = false;
        }

        if is_key_released(KeyCode::A) {
            self.keypad[7] = false;
        }
        if is_key_released(KeyCode::S) {
            self.keypad[8] = false;
        }
        if is_key_released(KeyCode::D) {
            self.keypad[9] = false;
        }
        if is_key_released(KeyCode::F) {
            self.keypad[0xE] = false;
        }

        if is_key_released(KeyCode::Z) {
            self.keypad[0xA] = false;
        }
        if is_key_released(KeyCode::X) {
            self.keypad[0] = false;
        }
        if is_key_released(KeyCode::C) {
            self.keypad[0xB] = false;
        }
        if is_key_released(KeyCode::V) {
            self.keypad[0xF] = false;
        }
    }

    pub fn fetch_instruction(&mut self) -> Instruction {
        let bytes: [u8; 2] = [self.memory[self.pc], self.memory[self.pc + 1]];
        println!("Bytes read: {:02x} {:02x}", bytes[0], bytes[1]);


        let result = u16::from_be_bytes(bytes);
        println!("Result {:04x}", result);
        self.pc += 2;
        return Instruction::new(result);
    }

    pub fn decode(&mut self, instr: Instruction, config: &crate::config::Config) {
        let bit_shift = instr.d();
        println!("{bit_shift:x}");
        match bit_shift {
            0x0 => match instr.nnn() {
                // 0x0E0: clear screen
                0x0E0 => self.display = [[false; 32]; 64],
                0x0EE => self.pc = self.stack.pop().expect("ERROR: returning from subroutine that never happened") as usize,
                _ => println!("Opcode: 0x0NNN is not implemented in this emulator"),
            },
            0x1 => {
                // 0x1NNN: jump to NNN
                let address = instr.nnn();
                println!("address jump: {:x}", address);
                self.pc = address as usize;
            },
            0x2 => {
                // 0x2NNN: call subroutine at NNN, push PC to stack first
                self.stack.push(self.pc as u16);
                self.pc = instr.nnn() as usize;
            },
            0x3 => {
                // 0x3XNN: If VX == NN: pc += 2
                if self.V[instr.x() as usize] as u16 == instr.nn() {
                    self.pc += 2;
                }
            },
            0x4 => {
                // 0x4XNN: If VX != NN: pc += 2
                if self.V[instr.x() as usize] as u16 != instr.nn() {
                    self.pc += 2;
                }
            },
            0x5 => {
                // 0x5XY0: If VX == VY: pc += 2
                let x = instr.x() as usize;
                let y = instr.y() as usize;
                if self.V[x] == self.V[y] {
                    self.pc += 2;
                }
            },
            0x6 => {
                // 0x6XNN: set VX to NN
                let x: usize = instr.x().into();
                self.V[x] = (instr.nn()) as u8;
                println!("Set V{:01x} to {:x}", x, self.V[x]);
            },
            0x7 => {
                // 0x7XNN: add NN to VX
                let x: usize = instr.x().into();
                self.V[x] = self.V[x].overflowing_add(instr.nn() as u8).0;
            },
            0x8 => {
                // Logic & Arithmetic instructions
                let bit_shift = instr.n();
                match bit_shift {
                    0x0 => {
                        // 0x8XY0: VX set to VY
                        let x = instr.x() as usize;
                        let y = instr.y() as usize;
                        self.V[x] = self.V[y];
                    },
                    0x1 => {
                        // 8XY1: VX = VX OR VY
                        let x = instr.x() as usize;
                        let y = instr.y() as usize;
                        self.V[x] |= self.V[y];
                    },
                    0x2 => {
                        // 8XY2: VX = VX AND VY
                        let x = instr.x() as usize;
                        let y = instr.y() as usize;
                        self.V[x] &= self.V[y];
                    },
                    0x3 => {
                        // 8XY3: VX = VX XOR VY
                        let x = instr.x() as usize;
                        let y = instr.y() as usize;
                        self.V[x] ^= self.V[y];
                    },
                    0x4 => {
                        // 8XY4: VX = VX ADD VY, if overflow: VF = 1 else VF = 0 
                        let x = instr.x() as usize;
                        let y = instr.y() as usize;
                        let carry;
                        (self.V[x], carry) = self.V[x].overflowing_add(self.V[y]);
                        if carry {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                    },
                    0x5 => {
                        // 8XY5: VX = VX - VY, If VX > VY: VF = 1, else VF = 0
                        let x = instr.x() as usize;
                        let y = instr.y() as usize;
                        let carry;
                        (self.V[x], carry) = self.V[x].overflowing_sub(self.V[y]);
                        if !carry {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                    },
                    0x6 => {
                        // TODO: When implementing configuration, add the legacy version of this
                        // opcode

                        // 8XY6: VX bit shift 1 to the right
                        let x = instr.x() as usize;
                        let carry = self.V[instr.x() as usize] & 1;
                        self.V[x] >>= 1;
                        self.V[0xF] = carry;
                    },
                    0xE => {
                        // TODO: When implementing configuration, add the legacy version of this
                        // opcode

                        // 8XYE: VX but shift 1 to the left
                        let x = instr.x() as usize;
                        let carry = (self.V[x] & 0x80) >> 7;
                        self.V[x] <<= 1;
                        self.V[0xF] = carry;
                    }
                    0x7 => {
                        // 8XY7: VX = VY - VX, If VY > VX: VF = 1, else VF = 0
                        let x = instr.x() as usize;
                        let y = instr.y() as usize;
                        let carry;
                        (self.V[x], carry) = self.V[y].overflowing_sub(self.V[x]);
                        if !carry {
                            self.V[0xF] = 1;
                        }
                        else {
                            self.V[0xF] = 0;
                        }
                    }
                    _ => unreachable!("Logical opcode does not exist"),
                }
            }
            0x9 => {
                // 0x9XY0: If VX != VY: pc +=2
                let x = instr.x() as usize;
                let y = instr.y() as usize as usize;
                if self.V[x] != self.V[y] {
                    self.pc += 2;
                }
            }
            0xA => {
                // ANNN: set I to the address of NNN
                let nnn = instr.nnn();
                self.I = nnn;
            },
            0xB => {
                // TODO: When implementing configuration, add the legacy version of this opcode

                // BNNN: PC += V0 + NNN
                self.pc += (self.V[0] as u16 + (instr.nnn())) as usize;
            },
            0xC => {
                // CXNN: VX = RandNum & NN
                let x = instr.x() as usize;
                let nn = instr.nn();
                self.V[x] = rand::gen_range(0, u8::MAX) & nn as u8;
            }
            0xD => {
                let x = instr.x() as usize;
                let y = instr.y() as usize as usize;

                let mut x_coord = self.V[x] as usize % config.get_raw_width() as usize;
                let mut y_coord = self.V[y] as usize % config.get_raw_height() as usize;
                let original_x = x_coord;

                self.V[0xF] = 0;

                let n = instr.n();
                for i in 0..n {
                    let sprite_data = self.memory[self.I as usize + i as usize];
                    x_coord = original_x;

                    for j in (0..8).rev() {
                        let pixel: &mut bool = &mut self.display[x_coord][y_coord];
                        let sprite_bit = (sprite_data & (1 << j)) != 0; // Use != 0 for clarity
                        if *pixel && sprite_bit {
                            *pixel = false;
                            self.V[0xF] = 1;
                        }
                        *pixel ^= sprite_bit;
                        x_coord = (x_coord + 1) % config.get_raw_width() as usize; // Wrap around
                    }
                    y_coord = (y_coord + 1) % config.get_raw_height() as usize; // Wrap around
                }
                self.set_draw(true);
            },
            0xE => {
                let bitmask = instr.nn();
                match bitmask {
                    0x9E => {
                        // EX9E: if key in V[X] is pressed; pc += 2
                        let x = instr.x() as usize;
                        let key_code = self.V[x] as usize;

                        if self.keypad[key_code] {
                            self.pc += 2;
                        }
                    },
                    0xA1 => {
                        // EXA1: if key in V[X] is not pressed; pc += 2
                        let x = instr.x() as usize;
                        let key_code = self.V[x] as usize;

                        if !self.keypad[key_code] {
                            self.pc += 2;
                        }
                    },
                    _ => panic!("opcode does not exist"),
                }
            },
            0xF => {
                let bitmask = instr.nn();
                match bitmask {
                    0x07 => {
                        // FX07: V[X] = delay_timer
                        let x = instr.x() as usize;
                        self.V[x] = self.delay_timer;
                    },
                    0x15 => {
                        // FX15: delay_timer = V[X]
                        self.delay_timer = self.V[instr.x() as usize];
                    },
                    0x18 => {
                        // FX18: sound_timer = VX
                        let x = instr.nn() as usize;
                        self.sound_timer = self.V[x];
                    },
                    0x1E => {
                        // FX1E: I += V[X]
                        let x = instr.x() as usize;
                        self.I += self.V[x] as u16;
                    },
                    0x0A => {
                        // FX0A: VX = get_key(); Await until a keypress and store in VX
                        let x = instr.x() as usize;
                        let mut any_key_pressed = false;
                        for (i, _) in self.keypad.iter().enumerate() {
                            if self.keypad[i] {
                                self.V[x] = i as u8;
                                any_key_pressed = true;
                                break;
                            }
                        }
                        if !any_key_pressed {
                            self.pc -= 2;
                        }

                    },
                    0x29 => {
                        // FX29: I = V[X] * 5
                        let x = instr.x() as usize;
                        self.I = self.V[x] as u16 * 5;
                    },
                    0x33 => {
                        // FX33: VX -> three decimal digits, stored at memory[I]
                        let mut bcd = self.V[instr.x() as usize];
                        self.memory[self.I as usize + 2] = bcd % 10;
                        bcd /= 10;
                        self.memory[self.I as usize + 1] = bcd % 10;
                        bcd /= 10;
                        self.memory[self.I as usize] = bcd;
                    },
                    0x55 => {
                        // FX55: V0 -> VX stored in memory[I] -> memory[I+X]
                        let dump = &self.V[0..=instr.x() as usize];
                        self.memory[self.I as usize..=self.I as usize + instr.x() as usize]
                            .copy_from_slice(dump);
                    },
                    0x65 => {
                        let x = instr.x() as usize;
                        for i in 0..=x {
                            self.V[i] = self.memory[self.I as usize + i];
                        }
                    },
                    _ => panic!("Opcode does not exist"),
                }
            }
            _ => {}
        }
    }
    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), &'static str> {
        if rom.len() == 0 {
            return Err("File is of len 0");
        }
        const ROM_START_LOCATION: usize = 0x200;
        // copy rom into memory
        self.memory[ROM_START_LOCATION..(ROM_START_LOCATION + rom.len())] 
            .copy_from_slice(rom);

        Ok(())
    }
    pub fn draw(&self, config: &crate::config::Config) {
        // Get the display data once
        let display = self.get_display();

        // Iterate over display pixels
        macroquad::prelude::scene::clear();
        for (i, row) in display.iter().enumerate() {
            for (j, &pixel_on) in row.iter().enumerate() {
                if pixel_on {
                    // Create point for drawing
                    let v = Vec2::create_point(j as f32, i as f32, config);

                    // Draw rectangle representing the pixel
                    draw_rectangle(v.y, v.x, config.scale_factor, config.scale_factor, WHITE);
                }
            }
        }
    }
}


// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
// TEST
// ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;

    #[test]
    fn test_fx55() {
        let mut chip8 = Cpu::new();
        chip8.I = 0x400; // Set I to some address
        chip8.V[0] = 0x01;
        chip8.V[1] = 0x02;
        chip8.V[2] = 0x03;
        chip8.V[3] = 0x04; // Add additional registers if needed

        // Instruction FX55 where X is 2 (store V0 to V2)
        let instr = Instruction::new(0xF255); 
        let config = config::Config::default();
        chip8.decode(instr, &config);

        // Assert memory content
        assert_eq!(chip8.memory[0x400], 0x01);
        assert_eq!(chip8.memory[0x401], 0x02);
        assert_eq!(chip8.memory[0x402], 0x03);

        // Check that memory beyond the affected range is not altered
        assert_ne!(chip8.memory[0x403], 0x04);
    }
#[test]
    fn test_fx33() {
        let mut chip8 = Cpu::new();
        chip8.I = 0x300; // Set I to some address
        chip8.V[1] = 123; // Set V1 to 123

        // Instruction FX33 where X is 1 (convert V1 to BCD and store at I)
        let instr = Instruction::new(0xF133);
        let config = config::Config::default();
        chip8.decode(instr, &config);

        // Assert memory content
        assert_eq!(chip8.memory[0x300], 1); // Hundreds place
        assert_eq!(chip8.memory[0x301], 2); // Tens place
        assert_eq!(chip8.memory[0x302], 3); // Units place
    }
}
