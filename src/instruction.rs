/// an instruction can be made of multiple parts,
/// n being the first nibble of the instruction,
/// nn being the first two nibbles of the instruction,
/// and nnn being the first three nibbles.
/// However not every instruction uses up to nnn and instead
/// uses x and/or y. x is the third nibble and is commonly used to 
/// enter the VX register
/// y is similar in that it is used to enter the VY register, but is the second nibble.
/// d denotes the last nibble of the instruction, and is used to identify the opcode
pub struct Instruction {
    opcode: u16
}

impl Instruction {
    pub fn new(opcode: u16) -> Self {
        Self {
            opcode,
        }
    }
    /// see [Instruction] for information
    pub fn nnn(&self) -> u16 {
        self.opcode & 0xFFF
    }
    pub fn nn(&self) -> u16 {
        self.opcode & 0xFF
    }
    pub fn n (&self) -> u16 {
        self.opcode & 0xF
    }
    pub fn x (&self) -> u8 {
        (self.opcode >> 8) as u8 & 0xF
    }
    pub fn y (&self) -> u8 {
        (self.opcode >> 4) as u8 & 0xF
    }
    pub fn d (&self) -> u8 {
        (self.opcode >> 12) as u8 & 0x0F
    }
}
