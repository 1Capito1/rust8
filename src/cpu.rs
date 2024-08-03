// non_snake_case allowed for I and V registers
use macroquad::prelude::*;

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

    pub fn fetch_instruction(&mut self) -> u16 {
        let bytes: [u8; 2] = [self.memory[self.pc], self.memory[self.pc + 1]];
        println!("Bytes read: {:02x} {:02x}", bytes[0], bytes[1]);


        let result = u16::from_be_bytes(bytes);
        println!("Result {:04x}", result);
        self.pc += 2;
        return result;
    }

    pub fn decode(&mut self, instr: u16, config: &crate::config::Config) {
        let bit_shift = (instr >> 12) & 0x0F;
        println!("{bit_shift:x}");
        match bit_shift {
            0x0 => match instr & 0x0FFF {
                // 0x0E0: clear screen
                0x0E0 => self.display = [[false; 32]; 64],
                0x0EE => self.pc = self.stack.pop().expect("ERROR: returning from subroutine that never happened") as usize,
                _ => println!("Opcode: 0x0NNN is not implemented in this emulator"),
            },
            0x1 => {
                // 0x1NNN: jump to NNN
                let address = instr & 0xFFF;
                println!("address jump: {:x}", address);
                self.pc = address as usize;
            },
            0x2 => {
                // 0x2NNN: call subroutine at NNN, push PC to stack first
                self.stack.push(self.pc as u16);
                self.pc = instr as usize & 0xFFF;
            },
            0x3 => {
                // 0x3XNN: If VX == NN: pc += 2
                let x = (instr >> 8) & 0xF;
                let nn = instr & 0xFF;
                if x == nn {
                    self.pc += 2;
                }
            },
            0x4 => {
                // 0x4XNN: If VX != NN: pc += 2
                let x = (instr >> 8) & 0xF;
                let nn = instr & 0xFF;
                if x != nn {
                    self.pc += 2;
                }
            },
            0x5 => {
                // 0x5XY0: If VX == VY: pc += 2
                let x = (instr >> 8) as usize & 0xF;
                let y = (instr >> 4) as usize & 0xF;
                if self.V[x] == self.V[y] {
                    self.pc += 2;
                }
            },
            0x6 => {
                // 0x6XNN: set VX to NN
                let vx: usize = (instr >> 8) as usize & 0xF;
                self.V[vx] = (instr & 0xFF) as u8;
                println!("Set V{:01x} to {:x}", vx, self.V[vx]);
            },
            0x7 => {
                // 0x7XNN: add NN to VX
                let vx: usize = (instr >> 8) as usize & 0xF;
                self.V[vx] += (instr & 0xFF) as u8;
            },
            0x8 => {
                // Logic & Arithmetic instructions
                let bit_shift = instr & 0xF;
                match bit_shift {
                    0x0 => {
                        // 0x8XY0: VX set to VY
                        let x = (instr >> 8) as usize & 0xF;
                        let y = (instr >> 4) as usize & 0xF;
                        self.V[x] = self.V[y];
                    },
                    0x1 => {
                        // 8XY1: VX = VX OR VY
                        let x = (instr >> 8) as usize & 0xF;
                        let y = (instr >> 4) as usize & 0xF;
                        self.V[x] |= self.V[y];
                    },
                    0x2 => {
                        // 8XY2: VX = VX AND VY
                        let x = (instr >> 8) as usize & 0xF;
                        let y = (instr >> 4) as usize & 0xF;
                        self.V[x] &= self.V[y];
                    },
                    0x3 => {
                        // 8XY3: VX = VX XOR VY
                        let x = (instr >> 8) as usize & 0xF;
                        let y = (instr >> 4) as usize & 0xF;
                        self.V[x] ^= self.V[y];
                    },
                    0x4 => {
                        // 8XY4: VX = VX ADD VY, if overflow: VF = 1 else VF = 0 
                        let x = (instr >> 8) as usize & 0xF;
                        let y = (instr >> 4) as usize & 0xF;
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
                        let x = (instr >> 8) as usize & 0xF;
                        let y = (instr >> 4) as usize & 0xF;
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
                        let x = (instr >> 8) as usize & 0xF;
                        self.V[x] >>= 1;
                    },
                    0xE => {
                        // TODO: When implementing configuration, add the legacy version of this
                        // opcode

                        // 8XYE: VX but shift 1 to the left
                        let x = (instr >> 8) as usize & 0xF;
                        self.V[x] <<= 1;
                    }
                    0x7 => {
                        // 8XY7: VX = VY - VX, If VY > VX: VF = 1, else VF = 0
                        let x = (instr >> 8) as usize & 0xF;
                        let y = (instr >> 4) as usize & 0xF;
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
                let x = (instr >> 8) as usize & 0xF;
                let y = (instr >> 4) as usize & 0xF;
                if self.V[x] != self.V[y] {
                    self.pc += 2;
                }
            }
            0xA => {
                // ANNN: set I to the address of NNN
                let nnn = instr & 0xFFF;
                self.I = nnn;
            },
            0xB => {
                // TODO: When implementing configuration, add the legacy version of this opcode
                
                // BNNN: PC += V0 + NNN
                self.pc += (self.V[0] as u16 + (instr & 0xFFF)) as usize;
            },
            0xC => {
                // CXNN: VX = RandNum & NN
                let x = (instr >> 8) as usize & 0xF;
                // TODO: self.V[x] = u8::rand
            }
            0xD => {
                let x = (instr >> 8) as usize & 0xF;
                let y = (instr >> 4) as usize & 0xF;

                let mut x_coord = self.V[x] as usize % config.get_raw_width() as usize;
                let mut y_coord = self.V[y] as usize % config.get_raw_height() as usize;
                let original_x = x_coord;

                self.V[0xF] = 0;

                let n = instr & 0xF;
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
            }
            _ => todo!(),
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
}
