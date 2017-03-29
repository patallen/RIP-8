#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    SysAddressJump_0x0NNN,          // Jump to address NNN
    ClearDisplay_0x00E0,            // Clear the display
    RetFromSubroutine_0x00EE,       // Return from Subroutine
    JumpLocation_0x1NNN,            // Jump to address: Set PC to 0xNNN
    CallSubroutine_0x2NNN,          // Call Subroutine: Set PC to 0xNNN, set sp += 1, set pc = NNN
    SkipInstrIfVxEqPL_0x3XNN,       // Skip Instruction if v[x] == 0xNN
    SkipInstrIfVxNotEqPL_0x4XNN,    // Skip Instruction if v[x] != 0xNN
    SkipInstrIfVxVy_0x5XY0,         // Skip Instruction if v[x] == v[y]
    SetVxToPL_0x6XNN,               // Set v[x] to 0xNN
    IncrementVxByPL_0x7XNN,         // Increment v[x] by 0xNN
    SetVxToVy_0x8XY0,               // Set v[x] to v[y](xx)
    SetVxToVxORVy_0x8XY1,           // Set v[x] to v[x] | v[y]
    SetVxToVxANDVy_0x8XY2,          // Set v[x] to v[x] & v[y]
    SetVxToVxXORVy_0x8XY3,          // Set v[x] to v[x] ^ v[y]
    IncrementVxByVyAndCarry_0x8XY4, // Increment v[x](xx) by v[y](yy) and set v[F] = 1 if overflow
    DecrementVxByVyNoBorrow_0x8XY5, // Decrement v[x](xx) by v[y](yy) and set v[F] = 1 if v[x] > v[y]
    ShiftAndRotateVxRight_0x8XY6,   // Shift and rotate v[x] right
    DecrementVyByVxNoBorrow_0x8XY7, // Decrement v[y](yy) by v[x](xx) and set v[F] = 1 if v[y] > v[x]
    ShiftAndRotateVxLeft_0x8XYE,    // Shift and rotate v[x] left
    SkipInstrIfVxNotVy_0x9XY0,      // Skip instruction if v[x](xx) != v[y](yy)
    SetIndexRegToPL_0xANNN,         // Set index to 0xNNN
    JumpToV0PlusPL_0xBNNN,          // Jump to v[0] + 0xNNN: Set PC to 0xXXX
    SetVxRandByteANDPL_0xCXNN,      // Set v[x] to randbyte(0xNNN) & 0xNN
    DisplaySpriteSetVfColl_0xDXYN,  // Display N-byte sprite and set v[F] = 1 if collision
    SkipInstrIfVxPressed_0xEX9E,    // Skip instruction if v[x](keycode) pressed
    SkipInstrIfVxNotPressed_0xEXA1, // Skip instruction if v[x](keycode) not pressed
    SetVxToDelayTimerVal_0xFX07,    // Set v[x] to value of delay timer (xxx)
    WaitForKeyStoreInVx_0xFX0A,     // Wait for key and store it's value in v[x]
    SetDelayTimerToVx_0xFX15,       // Set delay timer to v[x](xx)
    SetSoundTimerToVx_0xFX18,       // Set sound timer to v[x](xx)
    IncrementIndexRegByVx_0xFX1E,   // Set index = index(xx) + v[x](xx)
    SetIndexRegToVxSprite_0xFX29,   // Set index equal to the v[x]th sprite (v[x] * 5)
    StoreBCDOfVxIn3Bytes_0xFX33,    // Store BCD of v[x](xxx) in mem[i], mem[i+1], mem[i+2]
    StoreRegsUptoVx_0xFX55,         // Store v[0] through v[x] in mem[i] through mem[i + x]
    ReadRegsUptoVx_0xFX65,          // Store mem[i] through mem[i+x] in v[0] through v[x]
}

pub struct Opcode {
    pub value: u16,
    pub instr: Instruction,
}

impl Opcode {
    pub fn from_bytes(b1: u8, b2: u8) -> Opcode {
        let value: u16 = (b1 as u16) << 8 | b2 as u16;
        Opcode {
            value: value,
            instr: parse_opcode(value).unwrap(),
        }
    }
    pub fn from_code(code: u16) -> Opcode {
        Opcode {
            value: code,
            instr: parse_opcode(code).unwrap(),
        }
    }
    pub fn x(&self) -> usize {
        (self.value >> 8 & 0xF) as usize
    }
    pub fn y(&self) -> usize {
        (self.value >> 4 & 0xF) as usize
    }
    pub fn z(&self) -> usize {
        (self.value & 0xF) as usize
    }
    pub fn yz(&self) -> u16 {
        self.value & 0xFF
    }
    pub fn xyz(&self) -> u16 {
        self.value & 0xFFF
    }
    pub fn parse(&self) -> (usize, usize, usize) {
        (self.x(), self.y(), self.z())
    }
}

pub fn parse_opcode(code: u16) -> Result<Instruction, &'static str> {
    match code & 0xF000 {
        0x0000 =>
            match code & 0x00FF {
                0xE0 => Ok(Instruction::ClearDisplay_0x00E0),
                0xEE => Ok(Instruction::RetFromSubroutine_0x00EE),
                _    => Ok(Instruction::SysAddressJump_0x0NNN),
            },
        0x1000 => Ok(Instruction::JumpLocation_0x1NNN),
        0x2000 => Ok(Instruction::CallSubroutine_0x2NNN),
        0x3000 => Ok(Instruction::SkipInstrIfVxEqPL_0x3XNN),
        0x4000 => Ok(Instruction::SkipInstrIfVxNotEqPL_0x4XNN),
        0x5000 => Ok(Instruction::SkipInstrIfVxVy_0x5XY0),
        0x6000 => Ok(Instruction::SetVxToPL_0x6XNN),
        0x7000 => Ok(Instruction::IncrementVxByPL_0x7XNN),
        0x8000 =>
            match code & 0x000F {
                0x0 => Ok(Instruction::SetVxToVy_0x8XY0),
                0x1 => Ok(Instruction::SetVxToVxORVy_0x8XY1),
                0x2 => Ok(Instruction::SetVxToVxANDVy_0x8XY2),
                0x3 => Ok(Instruction::SetVxToVxXORVy_0x8XY3),
                0x4 => Ok(Instruction::IncrementVxByVyAndCarry_0x8XY4),
                0x5 => Ok(Instruction::DecrementVxByVyNoBorrow_0x8XY5),
                0x6 => Ok(Instruction::ShiftAndRotateVxRight_0x8XY6),
                0x7 => Ok(Instruction::DecrementVyByVxNoBorrow_0x8XY7),
                0xE => Ok(Instruction::ShiftAndRotateVxLeft_0x8XYE),
                _ => Err("Failed to get opcode at 0x8***")
            },
        0x9000 => Ok(Instruction::SkipInstrIfVxNotVy_0x9XY0),
        0xA000 => Ok(Instruction::SetIndexRegToPL_0xANNN),
        0xB000 => Ok(Instruction::JumpToV0PlusPL_0xBNNN),
        0xC000 => Ok(Instruction::SetVxRandByteANDPL_0xCXNN),
        0xD000 => Ok(Instruction::DisplaySpriteSetVfColl_0xDXYN),
        0xE000 => 
            match code & 0x00FF {
                0x9E => Ok(Instruction::SkipInstrIfVxPressed_0xEX9E),
                0xA1 => Ok(Instruction::SkipInstrIfVxNotPressed_0xEXA1),
                _ => Err("Failed to get opcode at 0xE***")
            },
        0xF000 => 
            match code & 0x00FF {
                0x07 => Ok(Instruction::SetVxToDelayTimerVal_0xFX07),
                0x0A => Ok(Instruction::WaitForKeyStoreInVx_0xFX0A),
                0x15 => Ok(Instruction::SetDelayTimerToVx_0xFX15),
                0x18 => Ok(Instruction::SetSoundTimerToVx_0xFX18),
                0x1E => Ok(Instruction::IncrementIndexRegByVx_0xFX1E),
                0x29 => Ok(Instruction::SetIndexRegToVxSprite_0xFX29),
                0x33 => Ok(Instruction::StoreBCDOfVxIn3Bytes_0xFX33),
                0x55 => Ok(Instruction::StoreRegsUptoVx_0xFX55),
                0x65 => Ok(Instruction::ReadRegsUptoVx_0xFX65),
                _ => Err("Failed to get opcode at 0xF***")
            },
        _ => Err("Could not get opcode.")
    }
}

#[test]
pub fn test_parse_opcode() {
    use std::collections::HashMap;
    use cpu::CPU;
    let mut cpu = CPU::new();
    let code_results: HashMap<u16, Instruction> = [
        (0x00EE, Instruction::RetFromSubroutine_0x00EE),
        (0x00E0, Instruction::ClearDisplay_0x00E0),
        (0x0000, Instruction::SysAddressJump_0x0000),
        (0x1000, Instruction::JumpLocation_0x1000),
        (0x2000, Instruction::CallSubroutine_0x2000),
        (0x3000, Instruction::SkipInstrIfVxEqPL_0x3000),
        (0x4000, Instruction::SkipInstrIfVxNotEqPL_0x4000),
        (0x5000, Instruction::SkipInstrIfVxVy_0x5000),
        (0x6000, Instruction::SetVxToPL_0x6000),
        (0x7000, Instruction::IncrementVxByPL_0x7000),
        (0x8FF0, Instruction::SetVxToVy_0x8000),
        (0x8FF1, Instruction::SetVxToVxORVy_0x8001),
        (0x8FF2, Instruction::SetVxToVxANDVy_0x8002),
        (0x8FF3, Instruction::SetVxToVxXORVy_0x8003),
        (0x8FF4, Instruction::IncrementVxByVyAndCarry_0x8004),
        (0x8FF5, Instruction::DecrementVxByVyNoBorrow_0x8005),
        (0x8FF6, Instruction::ShiftAndRotateVxRight_0x8006),
        (0x8FF7, Instruction::DecrementVyByVxNoBorrow_0x8007),
        (0x8FFE, Instruction::ShiftAndRotateVxLeft_0x800E),
        (0x9000, Instruction::SkipInstrIfVxNotVy_0x9000),
        (0xA000, Instruction::SetIndexRegToPL_0xA000),
        (0xB000, Instruction::JumpToV0PlusPL_0xB000),
        (0xC000, Instruction::SetVxRandByteANDPL_0xC000),
        (0xD000, Instruction::DisplaySpriteSetVfColl_0xD000),
        (0xEF9E, Instruction::SkipInstrIfVxPressed_0xE09E),
        (0xEFA1, Instruction::SkipInstrIfVxNotPressed_0xE0A1),
        (0xFF07, Instruction::SetVxToDelayTimerVal_0xF007),
        (0xFF0A, Instruction::WaitForKeyStoreInVx_0xF00A),
        (0xFF15, Instruction::SetDelayTimerToVx_0xF015),
        (0xFF18, Instruction::SetSoundTimerToVx_0xF018),
        (0xFF1E, Instruction::IncrementIndexRegByVx_0xF01E),
        (0xFF29, Instruction::SetIndexRegToVxSprite_0xF029),
        (0xFF33, Instruction::StoreBCDOfVxIn3Bytes_0xF033),
        (0xFF55, Instruction::StoreRegsUptoVx_0xF055),
        (0xFF65, Instruction::ReadRegsUptoVx_0xF065),
    ].iter().cloned().collect();

    for (code, res) in &code_results {
        let result = parse_opcode(*code).unwrap();
        assert_eq!(*res, parse_opcode(*code).unwrap());
    }
}