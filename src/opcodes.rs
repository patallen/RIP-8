#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    SysAddressJump_0x0NNN,
    ClearDisplay_0x00E0,
    RetFromSubroutine_0x00EE,
    JumpLocation_0x1NNN,
    CallSubroutine_0x2NNN,
    SkipInstrIfVxEqPL_0x3XNN,
    SkipInstrIfVxNotEqPL_0x4XNN,
    SkipInstrIfVxVy_0x5XY0,
    SetVxToPL_0x6XNN,
    IncrementVxByPL_0x7XNN,
    SetVxToVy_0x8XY0,
    SetVxToVxORVy_0x8XY1,
    SetVxToVxANDVy_0x8XY2,
    SetVxToVxXORVy_0x8XY3,
    IncrementVxByVyAndCarry_0x8XY4,
    DecrementVxByVyNoBorrow_0x8XY5,
    ShiftAndRotateVxRight_0x8XY6,
    DecrementVyByVxNoBorrow_0x8XY7,
    ShiftAndRotateVxLeft_0x8XYE,
    SkipInstrIfVxNotVy_0x9XY0,
    SetIndexRegToPL_0xANNN,
    JumpToV0PlusPL_0xBNNN,
    SetVxRandByteANDPL_0xCXNN,
    DisplaySpriteSetVfColl_0xDXYN,
    SkipInstrIfVxPressed_0xEX9E,
    SkipInstrIfVxNotPressed_0xEXA1,
    SetVxToDelayTimerVal_0xFX07,
    WaitForKeyStoreInVx_0xFX0A,
    SetDelayTimerToVx_0xFX15,
    SetSoundTimerToVx_0xFX18,
    IncrementIndexRegByVx_0xFX1E,
    SetIndexRegToVxSprite_0xFX29,
    StoreBCDOfVxIn3Bytes_0xFX33,
    StoreRegsUptoVx_0xFX55,
    ReadRegsUptoVx_0xFX65,
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
    pub fn x(&mut self) -> usize {
        (self.value >> 8 & 0xF) as usize
    }
    pub fn y(&mut self) -> usize {
        (self.value >> 4 & 0xF) as usize
    }
    pub fn z(&mut self) -> usize {
        (self.value & 0xF) as usize
    }
    pub fn yz(&mut self) -> u16 {
        self.value & 0xFF
    }
    pub fn xyz(&mut self) -> u16 {
        self.value & 0xFFF
    }
    pub fn parse(&mut self) -> (usize, usize, usize) {
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