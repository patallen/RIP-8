extern crate rand;
extern crate sdl2;

use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use opcodes::{parse_opcode, OpCode};
use device::Device;

use ::{DEBUG, DEBUG_CHUNK};


pub struct CPU<'cpu> {
    pub mem: [u8; 4096],
    pub regs: [u8; 16],
    pub index: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub opcode: u16,
    pub pc: u16,
    pub device: Device<'cpu>,
}

const FONT_SET: [u8; 80] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

impl<'cpu> CPU <'cpu>{
    pub fn new() -> CPU<'cpu> {
        let mut cpu = CPU {
            mem:    [0; 4096],
            regs:   [0; 16],
            stack:  [0; 16],
            index:  0x200,
            opcode: 0,
            sp:     0, // Pointer to the topmost of the stack
            pc:     0x200,
            device: Device::new(),
        };
        cpu.set_fonts();
        cpu.opcode = cpu.opcode_at_address(cpu.pc as usize);
        cpu
    }
    pub fn run(&mut self) {
        let mut count: u16 = 0;
        loop {
            self.device.pump();
            if self.device.quit {
                break;
            }
            self.opcode = self.opcode_at_address(self.pc as usize);
            if DEBUG {
                let inst = parse_opcode(self.opcode).unwrap();
                println!("Instr: {:?}. Code: 0x{:X}. PC: 0x{:X}. SP: 0x{:X}. *SP: 0x{:X}. I: 0x{:X}\r", inst, self.opcode, self.pc, self.sp, self.stack[self.sp as usize], self.index);
                println!("REGS: r0:{:x}|r1:{:x}|r2:{:x}|r3:{:x}|r4:{:x}|r5:{:x}|r6:{:x}|r7:{:x}|r8:{:x}|r9:{:x}|rA:{:x}|rB:{:x}|rC:{:x}|rD:{:x}|rE:{:x}|rF:{:x}|", self.regs[0], self.regs[1], self.regs[2], self.regs[3], self.regs[4], self.regs[5], self.regs[6], self.regs[7], self.regs[8], self.regs[9], self.regs[10], self.regs[11], self.regs[12], self.regs[13] , self.regs[14], self.regs[15]);
                count += 1;
                if count >= DEBUG_CHUNK {
                    let mut s = String::new();
                    io::stdin().read_line(&mut s).unwrap();
                    count = 0;
                }
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
    }
    pub fn load_rom(&mut self, filepath: &str) {
        let mut rom: Vec<u8> = Vec::new();
        let mut file = File::open(filepath).unwrap();
        file.read_to_end(&mut rom);

        for (i, mut byte) in rom.iter().enumerate() {
            self.mem[i + 512] = *byte;
        }
    }
    fn set_fonts(&mut self) {
        for (i, byte) in FONT_SET.into_iter().enumerate() {
            self.mem[i] = *byte;
        }
    }
    pub fn opcode_at_address(&self, address: usize) -> u16 {
        let mut ret = self.mem[address] as u16;
        let ret2 = self.mem[address + 1] as u16;
        (ret << 8 | ret2)
    }
    pub fn run_opcode_instruction<'a>(&mut self, instrcode: OpCode) {
        match instrcode {
            OpCode::SysAddressJump_0x0NNN           =>  self.system_address_jump(),
            OpCode::ClearDisplay_0x00E0             =>  self.clear_display(),
            OpCode::RetFromSubroutine_0x00EE        =>  self.return_from_sub(),
            OpCode::JumpLocation_0x1NNN             =>  self.jump_to_location(),
            OpCode::CallSubroutine_0x2NNN           =>  self.call_subroutine(),
            OpCode::SkipInstrIfVxEqPL_0x3XNN        =>  self.skip_instr_if_vx_eq_pl(),
            OpCode::SkipInstrIfVxNotEqPL_0x4XNN     =>  self.skip_instr_if_vx_neq_pl(),
            OpCode::SkipInstrIfVxVy_0x5XY0          =>  self.skip_instr_if_vx_eq_vy(),
            OpCode::SetVxToPL_0x6XNN                =>  self.set_vx_to_pl(),
            OpCode::IncrementVxByPL_0x7XNN          =>  self.increment_vx_by_pl(),
            OpCode::SetVxToVy_0x8XY0                =>  self.set_vx_to_vy(),
            OpCode::SetVxToVxORVy_0x8XY1            =>  self.set_vx_to_vx_or_vy(),
            OpCode::SetVxToVxANDVy_0x8XY2           =>  self.set_vx_to_vx_and_vy(),
            OpCode::SetVxToVxXORVy_0x8XY3           =>  self.set_vx_to_vx_xor_vy(),
            OpCode::IncrementVxByVyAndCarry_0x8XY4  =>  self.increment_vx_by_vy_carry(),
            OpCode::DecrementVxByVyNoBorrow_0x8XY5  =>  self.decrenent_vx_by_vy_no_borrow(),
            OpCode::ShiftAndRotateVxRight_0x8XY6    =>  self.shift_and_rotate_vx_right(),
            OpCode::DecrementVyByVxNoBorrow_0x8XY7  =>  self.decrement_vy_by_vx_no_borrow(),
            OpCode::ShiftAndRotateVxLeft_0x8XYE     =>  self.shift_and_rotate_vx_left(),
            OpCode::SkipInstrIfVxNotVy_0x9XY0       =>  self.skip_instr_if_vx_not_vy(),
            OpCode::SetIndexRegToPL_0xANNN          =>  self.set_index_register_to_pl(),
            OpCode::JumpToV0PlusPL_0xBNNN           =>  self.jump_to_v0_plus_pl(),
            OpCode::SetVxRandByteANDPL_0xCXNN       =>  self.set_vx_rand_byte_and_pl(),
            OpCode::DisplaySpriteSetVfColl_0xDXYN   =>  self.display_sprite_set_vf_collision(),
            OpCode::SkipInstrIfVxPressed_0xEX9E     =>  self.skip_instr_if_vx_pressed(),
            OpCode::SkipInstrIfVxNotPressed_0xEXA1  =>  self.skip_instr_if_vx_not_pressed(),
            OpCode::SetVxToDelayTimerVal_0xFX07     =>  self.set_vs_to_delay_timer_val(),
            OpCode::WaitForKeyStoreInVx_0xFX0A      =>  self.wait_for_key_and_store_in_vx(),
            OpCode::SetDelayTimerToVx_0xFX15        =>  self.set_delay_timer_to_vx(),
            OpCode::SetSoundTimerToVx_0xFX18        =>  self.set_sound_timer_to_vx(),
            OpCode::IncrementIndexRegByVx_0xFX1E    =>  self.increment_index_register_by_vx(),
            OpCode::SetIndexRegToVxSprite_0xFX29    =>  self.set_index_register_to_vx_sprite(),
            OpCode::StoreBCDOfVxIn3Bytes_0xFX33     =>  self.store_bcd_of_vx_3bytes(),
            OpCode::StoreRegsUptoVx_0xFX55          =>  self.store_registers_through_vx(),
            OpCode::ReadRegsUptoVx_0xFX65           =>  self.read_registers_through_vx(),   
        }
    }
    fn system_address_jump(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn return_from_sub(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
        self.pc += 2;
    }
    fn clear_display(&mut self) {
        // self.device.clear_display();
        self.pc += 2;
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
        let vx = self.regs[(self.opcode >> 8 & 0x0F) as usize];
        let nn = self.opcode & 0x00FF;

        if vx == nn as u8 {
            self.pc += 2;
        }

        self.pc += 2;
    }
    fn skip_instr_if_vx_neq_pl(&mut self) {
        let vx = self.regs[(self.opcode >> 8 & 0x0F) as usize];
        let nn = self.opcode & 0x00FF;

        if vx != nn as u8 {
            self.pc += 2;
        }

        self.pc += 2;
    }
    fn skip_instr_if_vx_eq_vy(&mut self) {
        let vx = self.regs[(self.opcode >> 8 & 0x0F) as usize];
        let vy = self.regs[(self.opcode >> 4 & 0x0F) as usize];

        if vx == vy {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn set_vx_to_pl(&mut self) {
        let kk = self.opcode & 0x00FF;
        let x = self.opcode >> 8 & 0x0F;

        self.regs[x as usize] = kk as u8;
        self.pc += 2;
    }
    fn increment_vx_by_pl(&mut self) {
        let x = (self.opcode >> 8 & 0xF) as usize;
        let pl = (self.opcode & 0x00FF) as u16;

        let mut val = self.regs[x] as u16 + pl;
        if val > 255 {
            val = val - (val / 256 * 256) + 1;
        }

        self.regs[x] = val as u8;
        self.pc += 2
    }
    fn set_vx_to_vy(&mut self) {
        let x = self.opcode >> 8 & 0xF;
        let y = self.opcode >> 4 & 0xF;
        self.regs[x as usize] = self.regs[y as usize] as u8;
        self.pc += 2;
    }
    fn set_vx_to_vx_or_vy(&mut self) {
        let x = (self.opcode << 8 & 0x0F) as usize;
        let y = (self.opcode << 4 & 0xF) as usize;
        self.mem[x] = self.mem[x] | self.mem[y];
        self.pc += 2;
    }
    fn set_vx_to_vx_and_vy(&mut self) {
        let x = (self.opcode << 8 & 0x0F) as usize;
        let y = (self.opcode << 4 & 0xF) as usize;
        self.mem[x] = self.mem[x] & self.mem[y];
        self.pc += 2;
    }
    fn set_vx_to_vx_xor_vy(&mut self) {
        let x = (self.opcode << 8 & 0x0F) as usize;
        let y = (self.opcode << 4 & 0xF) as usize;
        self.mem[x] = self.mem[x] ^ self.mem[y];
        self.pc += 2;
    }
    fn increment_vx_by_vy_carry(&mut self) {
        let x = (self.opcode >> 8 & 0x0F) as usize;
        let vx = self.regs[x] as u16;
        let vy = self.regs[(self.opcode >> 8 & 0x0F) as usize] as u16;

        let mut val: u16 = vx + vy;
        let mut carry = 0;
        if val > 255 {
            val = val - (val / 256 * 256) + 1;
            carry = 1;
        }
        self.regs[x] = val as u8;
        self.regs[0xF] = carry as u8;

        self.pc += 2;
    }
    fn decrenent_vx_by_vy_no_borrow(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn shift_and_rotate_vx_right(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn decrement_vy_by_vx_no_borrow(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn shift_and_rotate_vx_left(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn skip_instr_if_vx_not_vy(&mut self) {
        // println!("Not Implemented.");
        self.pc += 4; // TODO: Change this
    }
    fn set_index_register_to_pl(&mut self) {
        let opcode = self.opcode;
        self.index = opcode & 0x0FFF;
        self.pc += 2;
    }
    fn jump_to_v0_plus_pl(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn set_vx_rand_byte_and_pl(&mut self) {
        let random: u8 = rand::random();
        let pl = self.opcode & 0x00FF;
        self.regs[(self.opcode >> 8 & 0x0F) as usize] = random & pl as u8;
        self.pc += 2;
    }
    fn display_sprite_set_vf_collision(&mut self) {
        // Dxyn - DRW Vx, Vy, nibble
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

        // The interpreter reads n bytes from memory, starting at the address stored in I. These
        // bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed
        // onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise
        // it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display,
        // it wraps around to the opposite side of the screen. 
        // let x = self.regs[(self.opcode >> 8 & 0x0F) as usize];
        // let y = self.regs[(self.opcode >> 4 & 0x0F) as usize];
        // let n = self.opcode & 0x0F;
        // let mut flag = false;

        // for i in 0..n {
        //     let byte = self.mem[self.index as usize + i as usize];
        //     let res = self.display.write_byte(byte, x as usize, y as usize + i as usize);
        //     flag = flag || res;
        // };

        // match flag {
        //     true => self.regs[0xF] = 1,
        //     false => self.regs[0xF] = 0,
        // }

        // self.display.draw();
        self.pc += 2;
    }
    fn skip_instr_if_vx_pressed(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn skip_instr_if_vx_not_pressed(&mut self) {
        println!("Not Implemented.");
        self.pc += 4;
    }
    fn set_vs_to_delay_timer_val(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn wait_for_key_and_store_in_vx(&mut self) {
        let idx = self.opcode >> 8 & 0x0F;
        self.regs[idx as usize] = 10;
        self.pc += 2;
    }
    fn set_delay_timer_to_vx(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn set_sound_timer_to_vx(&mut self) {
        // println!("Not Implemented.");
        self.pc += 2;
    }
    fn increment_index_register_by_vx(&mut self) {
        let idx = self.opcode >> 8 & 0x0F;
        self.index += self.regs[idx as usize] as u16;
        self.pc += 2;
    }
    fn set_index_register_to_vx_sprite(&mut self) {
        let vx = self.regs[(self.opcode >> 8 & 0x0F) as usize];
        self.index = (vx * 5) as u16;
        self.pc += 2;
    }
    fn store_bcd_of_vx_3bytes(&mut self) {
        let vx = self.regs[(self.opcode >> 8 & 0x0F) as usize];
        let hunds = vx / 100 * 100;
        let tens = (vx - hunds) / 10 * 10;
        let ones = (vx - hunds - tens);
        self.mem[self.index as usize] = hunds / 100;
        self.mem[self.index as usize + 1] = tens / 10;
        self.mem[self.index as usize + 2] = ones;

        self.pc += 2;
    }
    fn store_registers_through_vx(&mut self) {
        let x = self.opcode >> 8 & 0x0F;;

        for i in 0..x + 1 {
            let ii = i as usize;
            self.mem[self.index as usize + ii as usize] = self.regs[ii];
        }
        self.index = self.index + x;
        self.pc += 2;
    }
    fn read_registers_through_vx(&mut self) {
        // read memory self.mem[self.index : PLx] into registers starting at 0
        let opcode = self.opcode;
        let shifted = opcode >> 8 & 0x0F;
        let value = shifted as usize;
        let index = self.index as usize;

        for i in 0..value + 1{
            self.regs[i] = self.mem[index + i];
        }
        self.pc += 2;
    }
}

pub fn get_sub_arr(arr: &[u8; 4096], start: usize) -> [u8; 8] {
    let mut list: [u8; 8] = [0; 8];
    for i in 0..8 {
        list[i] = arr[i + start];
    }
    list
}

pub fn print_sprite(arr: &[u8; 4096], start: usize, no_bytes: usize) {
    let mut list: [u8; 8] = [0; 8];
    for i in 0..no_bytes {
        println!("{:b}", arr[i + start]);
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