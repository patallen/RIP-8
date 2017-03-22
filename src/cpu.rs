use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use opcodes::{parse_opcode, OpCode};
use ::DEBUG;


pub struct CPU {
    pub mem: [u8; 4096],
    pub regs: [u8; 16],
    pub index: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub opcode: u16,
    pub pc: u16,
}


impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            mem: [0; 4096],
            regs: [0; 16],
            index: 0,
            stack: [0; 16],
            sp: 0, // Pointer to the topmost of the stack
            opcode: 0,
            pc: 0x200
        };
        cpu.opcode = cpu.opcode_at_address(cpu.pc as usize);
        cpu
    }
    pub fn run(&mut self) {
        loop {
            self.opcode = self.opcode_at_address(self.pc as usize);
            if DEBUG {
                let inst = parse_opcode(self.opcode).unwrap();
                println!("Instr: {:?}. Code: 0x{:X}. PC: 0x{:X}. SP: 0x{:X}. *SP: 0x{:X}.\r", inst, self.opcode, self.pc, self.sp, self.stack[self.sp as usize]);
                let mut s = String::new();
                io::stdin().read_line(&mut s).unwrap();
            }
            self.cycle();
        }
    }
    pub fn cycle(&mut self) {
        let inst = parse_opcode(self.opcode);

        match inst {
            Ok(code) => self.run_opcode_instruction(code),
            Err(e) => panic!("{}", e),
        };

        self.pc += 2;
    }
    pub fn load_rom(&mut self, filepath: &str) {
        let mut rom: Vec<u8> = Vec::new();
        let mut file = File::open(filepath).unwrap();
        file.read_to_end(&mut rom);

        for (i, mut byte) in rom.iter().enumerate() {
            self.mem[i + 512] = *byte;
        }
    }
    pub fn opcode_at_address(&self, address: usize) -> u16 {
        let mut ret = self.mem[address] as u16;
        let ret2 = self.mem[address + 1] as u16;
        (ret << 8 | ret2)
    }
    pub fn run_opcode_instruction<'a>(&mut self, instrcode: OpCode) {
        match instrcode {
            SysAddressJump_0x0000           =>  self.system_address_jump(),
            ClearDisplay_0x00E0             =>  self.clear_display(),
            RetFromSubroutine_0x00EE        =>  self.return_from_sub(),
            JumpLocation_0x1000             =>  self.jump_to_location(),
            CallSubroutine_0x2000           =>  self.call_subroutine(),
            SkipInstrIfVxEqPL_0x3000        =>  self.skip_instr_if_vx_eq_pl(),
            SkipInstrIfVxNotEqPL_0x4000     =>  self.skip_instr_if_vs_neq_pl(),
            SkipInstrIfVxVy_0x5000          =>  self.skip_instr_if_vx_eq_vy(),
            SetVxToPL_0x6000                =>  self.set_vs_to_pl(),
            IncrementVxByPL_0x7000          =>  self.increment_vx_by_pl(),
            SetVxToVy_0x8000                =>  self.set_vx_to_vy(),
            SetVxToVxORVy_0x8001            =>  self.set_vx_to_vx_or_vy(),
            SetVxToVxANDVy_0x8002           =>  self.set_vx_to_vx_and_vy(),
            SetVxToVxXORVy_0x8003           =>  self.set_vx_to_vx_xor_vy(),
            IncrementVxByVyAndCarry_0x8004  =>  self.increment_vx_by_vy_carry(),
            DecrementVxByVyNoBorrow_0x8005  =>  self.decrenent_vx_by_vy_no_borrow(),
            ShiftAndRotateVxRight_0x8006    =>  self.shift_and_rotate_vx_right(),
            DecrementVyByVxNoBorrow_0x8007  =>  self.decrement_vy_by_vx_no_borrow(),
            ShiftAndRotateVxLeft_0x800E     =>  self.shift_and_rotate_vx_left(),
            SkipInstrIfVxNotVy_0x9000       =>  self.skip_instr_if_vx_not_vy(),
            SetIndexRegToPL_0xA000          =>  self.set_index_register_to_pl(),
            JumpToV0PlusPL_0xB000           =>  self.jump_to_v0_plus_pl(),
            SetVxRandByteANDPL_0xC000       =>  self.set_vx_rand_byte_and_pl(),
            DisplaySpriteSetVfColl_0xD000   =>  self.display_sprite_set_vf_collision(),
            SkipInstrIfVxPressed_0xE09E     =>  self.skip_instr_if_vx_pressed(),
            SkipInstrIfVxNotPressed_0xE0A1  =>  self.skip_instr_if_vx_not_pressed(),
            SetVxToDelayTimerVal_0xF007     =>  self.set_vs_to_delay_timer_val(),
            WaitForKeyStoreInVx_0xF00A      =>  self.wait_for_key_and_store_in_vx(),
            SetDelayTimerToVx_0xF015        =>  self.set_delay_timer_to_vx(),
            SetSoundTimerToVx_0xF018        =>  self.set_sound_timer_to_vx(),
            IncrementIndexRegByVx_0xF01E    =>  self.increment_index_register_by_vx(),
            SetIndexRegToVxSprite_0xF029    =>  self.set_index_register_to_vx_sprite(),
            StoreBCDOfVxIn3Bytes_0xF033     =>  self.store_bcd_of_vx_3bytes(),
            StoreRegsUptoVx_0xF055          =>  self.store_registers_through_vx(),
            ReadRegsUptoVx_0xF065           =>  self.read_registers_through_vx(),   
        }
    }
    fn system_address_jump(&mut self) {

    }
    fn return_from_sub(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -=1;
    }
    fn clear_display(&mut self) {

    }
    fn jump_to_location(&mut self) {
        // GOTO -> Set the PC to the specified address.
        // Doing so will make the interpreter pick up at this address
        // on the next cycle.
        let code = self.opcode;
        self.pc = code & 0x0FFF;
    }
    fn call_subroutine(&mut self) {
        let code = self.opcode;
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = code & 0x0FFF;
    }
    fn skip_instr_if_vx_eq_pl(&mut self) {
        // "0x3xnn"
    }
    fn skip_instr_if_vs_neq_pl(&mut self) {
        // "0x4xnn"
    }
    fn skip_instr_if_vx_eq_vy(&mut self) {
        // "0x5xy0"
    }
    fn set_vs_to_pl(&mut self) {

    }
    fn increment_vx_by_pl(&mut self) {
        // "0x7xnn"
    }
    fn set_vx_to_vy(&mut self) {

    }
    fn set_vx_to_vx_or_vy(&mut self) {

    }
    fn set_vx_to_vx_and_vy(&mut self) {

    }
    fn set_vx_to_vx_xor_vy(&mut self) {

    }
    fn increment_vx_by_vy_carry(&mut self) {

    }
    fn decrenent_vx_by_vy_no_borrow(&mut self) {

    }
    fn shift_and_rotate_vx_right(&mut self) {

    }
    fn decrement_vy_by_vx_no_borrow(&mut self) {

    }
    fn shift_and_rotate_vx_left(&mut self) {

    }
    fn skip_instr_if_vx_not_vy(&mut self) {

    }
    fn set_index_register_to_pl(&mut self) {

    }
    fn jump_to_v0_plus_pl(&mut self) {

    }
    fn set_vx_rand_byte_and_pl(&mut self) {

    }
    fn display_sprite_set_vf_collision(&mut self) {

    }
    fn skip_instr_if_vx_pressed(&mut self) {

    }
    fn skip_instr_if_vx_not_pressed(&mut self) {

    }
    fn set_vs_to_delay_timer_val(&mut self) {

    }
    fn wait_for_key_and_store_in_vx(&mut self) {

    }
    fn set_delay_timer_to_vx(&mut self) {

    }
    fn set_sound_timer_to_vx(&mut self) {

    }
    fn increment_index_register_by_vx(&mut self) {

    }
    fn set_index_register_to_vx_sprite(&mut self) {

    }
    fn store_bcd_of_vx_3bytes(&mut self) {

    }
    fn store_registers_through_vx(&mut self) {

    }
    fn read_registers_through_vx(&mut self) {

    }
}

#[test]
pub fn test_run_operation_for_goto() {
    // opcode = 0x10 << 8 | 0xF0 = 0x10F0
    // this means that our GOTO should take the last 3 hex
    // digits and put them into our program counter
    let mut cpu = CPU::new();
    cpu.mem[0x200] = 0x10;
    cpu.mem[0x201] = 0xF0;
    cpu.cycle();
    assert_eq!(0x0F0, cpu.pc);
}

#[test]
pub fn test_run_operation_for_call_sub() {
    let mut cpu = CPU::new();
    cpu.mem[0x200] = 0x21;
    cpu.mem[0x201] = 0x00;
    cpu.cycle();
    assert_eq!(cpu.sp, 1);
    assert_eq!(cpu.stack[cpu.sp as usize], 0x202);
    assert_eq!(cpu.pc, 0x100);
}

#[test]
pub fn test_return_from_sub() {
    let mut cpu = CPU::new();
    cpu.mem[0x200] = 0x23;
    cpu.mem[0x201] = 0x00;
    cpu.mem[0x300] = 0x00;
    cpu.mem[0x301] = 0xEE;
    cpu.cycle();
    cpu.cycle();
    assert_eq!(cpu.pc, 0x202);
    assert_eq!(cpu.sp, 0);
}