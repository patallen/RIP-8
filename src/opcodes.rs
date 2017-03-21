#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    SysAddressJump_0x0000,
    ClearDisplay_0x00E0,
    RetFromSubroutine_0x00EE,
    JumpLocation_0x1000,
    CallSubroutine_0x2000,
    SkipInstrIfVxEqPL_0x3000,
    SkipInstrIfVxNotEqPL_0x4000,
    SkipInstrIfVxVy_0x5000,
    SetVxToPL_0x6000,
    IncrementVxByPL_0x7000,
    SetVxToVy_0x8000,
    SetVxToVxORVy_0x8001,
    SetVxToVxANDVy_0x8002,
    SetVxToVxXORVy_0x8003,
    IncrementVxByVyAndCarry_0x8004,
    DecrementVxByVyNoBorrow_0x8005,
    ShiftAndRotateVxRight_0x8006,
    DecrementVyByVxNoBorrow_0x8007,
    ShiftAndRotateVxLeft_0x800E,
    SkipInstrIfVxNotVy_0x9000,
    SetIndexRegToPL_0xA000,
    JumpToV0PlusPL_0xB000,
    SetVxRandByteANDPL_0xC000,
    DisplaySpriteSetVfColl_0xD000,
    SkipInstrIfVxPressed_0xE09E,
    SkipInstrIfVxNotPressed_0xE0A1,
    SetVxToDelayTimerVal_0xF007,
    WaitForKeyStoreInVx_0xF00A,
    SetDelayTimerToVx_0xF015,
    SetSoundTimerToVx_0xF018,
    IncrementIndexRegByVx_0xF01E,
    SetIndexRegToVxSprite_0xF029,
    StoreBCDOfVxIn3Bytes_0xF033,
    StoreRegsUptoVx_0xF055,
    ReadRegsUptoVx_0xF065,
}

pub fn parse_opcode(code: u16) -> Result<OpCode, &'static str> {
    match code & 0xF000 {
        0x0000 =>
            match code & 0x00FF {
                0xE0 => Ok(OpCode::ClearDisplay_0x00E0),
                0xEE => Ok(OpCode::RetFromSubroutine_0x00EE),
                _    => Ok(OpCode::SysAddressJump_0x0000),
            },
        0x1000 => Ok(OpCode::JumpLocation_0x1000),
        0x2000 => Ok(OpCode::CallSubroutine_0x2000),
        0x3000 => Ok(OpCode::SkipInstrIfVxEqPL_0x3000),
        0x4000 => Ok(OpCode::SkipInstrIfVxNotEqPL_0x4000),
        0x5000 => Ok(OpCode::SkipInstrIfVxVy_0x5000),
        0x6000 => Ok(OpCode::SetVxToPL_0x6000),
        0x7000 => Ok(OpCode::IncrementVxByPL_0x7000),
        0x8000 =>
            match code & 0x000F {
                0x0 => Ok(OpCode::SetVxToVy_0x8000),
                0x1 => Ok(OpCode::SetVxToVxORVy_0x8001),
                0x2 => Ok(OpCode::SetVxToVxANDVy_0x8002),
                0x3 => Ok(OpCode::SetVxToVxXORVy_0x8003),
                0x4 => Ok(OpCode::IncrementVxByVyAndCarry_0x8004),
                0x5 => Ok(OpCode::DecrementVxByVyNoBorrow_0x8005),
                0x6 => Ok(OpCode::ShiftAndRotateVxRight_0x8006),
                0x7 => Ok(OpCode::DecrementVyByVxNoBorrow_0x8007),
                0xE => Ok(OpCode::ShiftAndRotateVxLeft_0x800E),
                _ => Err("Failed to get opcode at 0x8***")
            },
        0x9000 => Ok(OpCode::SkipInstrIfVxNotVy_0x9000),
        0xA000 => Ok(OpCode::SetIndexRegToPL_0xA000),
        0xB000 => Ok(OpCode::JumpToV0PlusPL_0xB000),
        0xC000 => Ok(OpCode::SetVxRandByteANDPL_0xC000),
        0xD000 => Ok(OpCode::DisplaySpriteSetVfColl_0xD000),
        0xE000 => 
            match code & 0x00FF {
                0x9E => Ok(OpCode::SkipInstrIfVxPressed_0xE09E),
                0xA1 => Ok(OpCode::SkipInstrIfVxNotPressed_0xE0A1),
                _ => Err("Failed to get opcode at 0xE***")
            },
        0xF000 => 
            match code & 0x00FF {
                0x07 => Ok(OpCode::SetVxToDelayTimerVal_0xF007),
                0x0A => Ok(OpCode::WaitForKeyStoreInVx_0xF00A),
                0x15 => Ok(OpCode::SetDelayTimerToVx_0xF015),
                0x18 => Ok(OpCode::SetSoundTimerToVx_0xF018),
                0x1E => Ok(OpCode::IncrementIndexRegByVx_0xF01E),
                0x29 => Ok(OpCode::SetIndexRegToVxSprite_0xF029),
                0x33 => Ok(OpCode::StoreBCDOfVxIn3Bytes_0xF033),
                0x55 => Ok(OpCode::StoreRegsUptoVx_0xF055),
                0x65 => Ok(OpCode::ReadRegsUptoVx_0xF065),
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
    let code_results: HashMap<u16, OpCode> = [
        (0x00EE, OpCode::RetFromSubroutine_0x00EE),
        (0x00E0, OpCode::ClearDisplay_0x00E0),
        (0x0000, OpCode::SysAddressJump_0x0000),
        (0x1000, OpCode::JumpLocation_0x1000),
        (0x2000, OpCode::CallSubroutine_0x2000),
        (0x3000, OpCode::SkipInstrIfVxEqPL_0x3000),
        (0x4000, OpCode::SkipInstrIfVxNotEqPL_0x4000),
        (0x5000, OpCode::SkipInstrIfVxVy_0x5000),
        (0x6000, OpCode::SetVxToPL_0x6000),
        (0x7000, OpCode::IncrementVxByPL_0x7000),
        (0x8FF0, OpCode::SetVxToVy_0x8000),
        (0x8FF1, OpCode::SetVxToVxORVy_0x8001),
        (0x8FF2, OpCode::SetVxToVxANDVy_0x8002),
        (0x8FF3, OpCode::SetVxToVxXORVy_0x8003),
        (0x8FF4, OpCode::IncrementVxByVyAndCarry_0x8004),
        (0x8FF5, OpCode::DecrementVxByVyNoBorrow_0x8005),
        (0x8FF6, OpCode::ShiftAndRotateVxRight_0x8006),
        (0x8FF7, OpCode::DecrementVyByVxNoBorrow_0x8007),
        (0x8FFE, OpCode::ShiftAndRotateVxLeft_0x800E),
        (0x9000, OpCode::SkipInstrIfVxNotVy_0x9000),
        (0xA000, OpCode::SetIndexRegToPL_0xA000),
        (0xB000, OpCode::JumpToV0PlusPL_0xB000),
        (0xC000, OpCode::SetVxRandByteANDPL_0xC000),
        (0xD000, OpCode::DisplaySpriteSetVfColl_0xD000),
        (0xEF9E, OpCode::SkipInstrIfVxPressed_0xE09E),
        (0xEFA1, OpCode::SkipInstrIfVxNotPressed_0xE0A1),
        (0xFF07, OpCode::SetVxToDelayTimerVal_0xF007),
        (0xFF0A, OpCode::WaitForKeyStoreInVx_0xF00A),
        (0xFF15, OpCode::SetDelayTimerToVx_0xF015),
        (0xFF18, OpCode::SetSoundTimerToVx_0xF018),
        (0xFF1E, OpCode::IncrementIndexRegByVx_0xF01E),
        (0xFF29, OpCode::SetIndexRegToVxSprite_0xF029),
        (0xFF33, OpCode::StoreBCDOfVxIn3Bytes_0xF033),
        (0xFF55, OpCode::StoreRegsUptoVx_0xF055),
        (0xFF65, OpCode::ReadRegsUptoVx_0xF065),
    ].iter().cloned().collect();

    for (code, res) in &code_results {
        let result = parse_opcode(*code).unwrap();
        assert_eq!(*res, parse_opcode(*code).unwrap());
    }
}