extern crate rand;
extern crate sdl2;

use std::fs::File;
use std::io::{self, Read};
use std::collections::HashMap;
use opcodes::{parse_opcode, Instruction, Opcode};
use device::Device;
use utils::Timer;
use std::thread::sleep;
use std::time::{Duration, Instant};

use ::{DEBUG, DEBUG_CHUNK, DO_CHUNK_DEBUG};

#[derive(PartialEq)]
enum DebugMode {
    Off,
    Chunk,
    Step,
    Stream,
}
pub struct CPU<'cpu> {
    pub hz: u32,
    pub program_delay: Duration,
    pub mem: [u8; 4096],
    pub regs: [u8; 16],
    pub index: u16,
    pub stack: [u16; 16],
    pub sp: u8,
    pub opcode: Opcode,
    pub pc: u16,
    pub device: Device<'cpu>,
    pub delay_timer: Timer,
    pub sound_timer: Timer,
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
        let hz = 500;
        let pg = Duration::new(0, ((1.0 / hz as f64) * 1000000000.0) as u32);
        let mut cpu = CPU {
            hz: hz,
            program_delay: pg,
            mem:    [0; 4096],
            regs:   [0; 16],
            stack:  [0; 16],
            index:  0x200,
            opcode: Opcode::from_code(0),
            sp:     0, // Pointer to the topmost of the stack
            pc:     0x200,
            delay_timer: Timer::new(16_666_667),
            sound_timer: Timer::new(2_000_000),
            device: Device::new(),
        };
        cpu.set_fonts();
        cpu.opcode = cpu.opcode_at_address(0x200);
        cpu
    }
    pub fn reset(&mut self) {
        self.initialize();
        self.mem = [0; 4096];
        self.regs = [0; 16];
        self.stack = [0; 16];
        self.index = 0x200;
        self.sp = 0;
        self.pc = 0x200;
        self.delay_timer = Timer::new(16_666_667);
        self.sound_timer = Timer::new(2_000_000);
    }
    pub fn initialize(&mut self) {
        let pc = self.pc as usize;
        self.opcode = self.opcode_at_address(pc);
    }
    pub fn run(&mut self) {
        loop {
            if self.device.quit {
                break;
            }
            sleep(self.program_delay);
            self.cycle();
        }
    }
    pub fn set_speed_hz(&mut self, hertz: u32) {
        if hertz > 0 {
            warn!("{}, {:?}", hertz, self.program_delay);
            self.program_delay = Duration::new(0, ((1.0 / hertz as f64) * 1000000000.0) as u32);
            self.hz = hertz;
        }
    }
    pub fn cycle(&mut self) {
        self.device.pump();
        self.delay_timer.touch();
        self.sound_timer.touch();
        self.run_opcode_instruction();
        let pc = self.pc as usize;
        self.opcode = self.opcode_at_address(pc);
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
    pub fn opcode_at_address(&self, address: usize) -> Opcode {
        Opcode::from_bytes(self.mem[address], self.mem[address + 1])
    }
    pub fn run_opcode_instruction<'a>(&mut self) {
        match self.opcode.instr {
            Instruction::SysAddressJump_0x0NNN           =>  self.system_address_jump(),
            Instruction::ClearDisplay_0x00E0             =>  self.clear_display(),
            Instruction::RetFromSubroutine_0x00EE        =>  self.return_from_sub(),
            Instruction::JumpLocation_0x1NNN             =>  self.jump_to_location(),
            Instruction::CallSubroutine_0x2NNN           =>  self.call_subroutine(),
            Instruction::SkipInstrIfVxEqPL_0x3XNN        =>  self.skip_instr_if_vx_eq_pl(),
            Instruction::SkipInstrIfVxNotEqPL_0x4XNN     =>  self.skip_instr_if_vx_neq_pl(),
            Instruction::SkipInstrIfVxVy_0x5XY0          =>  self.skip_instr_if_vx_eq_vy(),
            Instruction::SetVxToPL_0x6XNN                =>  self.set_vx_to_pl(),
            Instruction::IncrementVxByPL_0x7XNN          =>  self.increment_vx_by_pl(),
            Instruction::SetVxToVy_0x8XY0                =>  self.set_vx_to_vy(),
            Instruction::SetVxToVxORVy_0x8XY1            =>  self.set_vx_to_vx_or_vy(),
            Instruction::SetVxToVxANDVy_0x8XY2           =>  self.set_vx_to_vx_and_vy(),
            Instruction::SetVxToVxXORVy_0x8XY3           =>  self.set_vx_to_vx_xor_vy(),
            Instruction::IncrementVxByVyAndCarry_0x8XY4  =>  self.increment_vx_by_vy_carry(),
            Instruction::DecrementVxByVyNoBorrow_0x8XY5  =>  self.decrenent_vx_by_vy_no_borrow(),
            Instruction::ShiftAndRotateVxRight_0x8XY6    =>  self.shift_and_rotate_vx_right(),
            Instruction::DecrementVyByVxNoBorrow_0x8XY7  =>  self.decrement_vy_by_vx_no_borrow(),
            Instruction::ShiftAndRotateVxLeft_0x8XYE     =>  self.shift_and_rotate_vx_left(),
            Instruction::SkipInstrIfVxNotVy_0x9XY0       =>  self.skip_instr_if_vx_not_vy(),
            Instruction::SetIndexRegToPL_0xANNN          =>  self.set_index_register_to_pl(),
            Instruction::JumpToV0PlusPL_0xBNNN           =>  self.jump_to_v0_plus_pl(),
            Instruction::SetVxRandByteANDPL_0xCXNN       =>  self.set_vx_rand_byte_and_pl(),
            Instruction::DisplaySpriteSetVfColl_0xDXYN   =>  self.display_sprite_set_vf_collision(),
            Instruction::SkipInstrIfVxPressed_0xEX9E     =>  self.skip_instr_if_vx_pressed(),
            Instruction::SkipInstrIfVxNotPressed_0xEXA1  =>  self.skip_instr_if_vx_not_pressed(),
            Instruction::SetVxToDelayTimerVal_0xFX07     =>  self.set_vx_to_delay_timer_val(),
            Instruction::WaitForKeyStoreInVx_0xFX0A      =>  self.wait_for_key_and_store_in_vx(),
            Instruction::SetDelayTimerToVx_0xFX15        =>  self.set_delay_timer_to_vx(),
            Instruction::SetSoundTimerToVx_0xFX18        =>  self.set_sound_timer_to_vx(),
            Instruction::IncrementIndexRegByVx_0xFX1E    =>  self.increment_index_register_by_vx(),
            Instruction::SetIndexRegToVxSprite_0xFX29    =>  self.set_index_register_to_vx_sprite(),
            Instruction::StoreBCDOfVxIn3Bytes_0xFX33     =>  self.store_bcd_of_vx_3bytes(),
            Instruction::StoreRegsUptoVx_0xFX55          =>  self.store_registers_through_vx(),
            Instruction::ReadRegsUptoVx_0xFX65           =>  self.read_registers_through_vx(),   
        }
    }
    fn system_address_jump(&mut self) {
        println!("Not Implemented.");

        self.pc += 2;
    }
    fn return_from_sub(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
        self.pc += 2;
    }
    fn clear_display(&mut self) {
        self.device.clear_display();
        self.pc += 2;
    }
    fn jump_to_location(&mut self) {
        self.pc = self.opcode.xyz();
    }
    fn call_subroutine(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = self.opcode.xyz();
    }
    fn skip_instr_if_vx_eq_pl(&mut self) {
        let vx = self.regs[self.opcode.x()];
        if vx == self.opcode.yz() as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn skip_instr_if_vx_neq_pl(&mut self) {
        let vx = self.regs[self.opcode.x()];

        if vx != self.opcode.yz() as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn skip_instr_if_vx_eq_vy(&mut self) {
        if self.regs[self.opcode.x()] == self.regs[self.opcode.y()] {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn set_vx_to_pl(&mut self) {
        self.regs[self.opcode.x()] = self.opcode.yz() as u8;
        self.pc += 2;
    }
    fn increment_vx_by_pl(&mut self) {
        let x = self.opcode.x();
        let pl = self.opcode.yz();

        self.regs[x] = ((self.regs[x] as u16 + pl) & 0xFFFF) as u8;
        self.pc += 2;
    }
    fn set_vx_to_vy(&mut self) {
        self.regs[self.opcode.x()] = self.regs[self.opcode.y()] as u8;
        self.pc += 2;
    }
    fn set_vx_to_vx_or_vy(&mut self) {
        let x = self.opcode.x();
        let y = self.opcode.y();
        self.regs[x] = self.regs[x] | self.regs[y];
        self.pc += 2;
    }
    fn set_vx_to_vx_and_vy(&mut self) {
        let x = self.opcode.x();
        let y = self.opcode.y();
        self.regs[x] = self.regs[x] & self.regs[y];
        self.pc += 2;
    }
    fn set_vx_to_vx_xor_vy(&mut self) {
        let x = self.opcode.x();
        let y = self.opcode.y();
        self.regs[x] = self.regs[x] ^ self.regs[y];
        self.pc += 2;
    }
    fn increment_vx_by_vy_carry(&mut self) {
        let vx = self.regs[self.opcode.x()] as u16;
        let vy = self.regs[self.opcode.y()] as u16;

        let mut val: u16 = vx + vy;
        let mut carry = 0;
        if val > 255 {
            val = val - (val / 256 * 256) + 1;
            carry = 1;
        }
        self.regs[self.opcode.x()] = val as u8;
        self.regs[0xF] = carry as u8;

        self.pc += 2;
    }
    fn decrenent_vx_by_vy_no_borrow(&mut self) {
        let vx = self.regs[self.opcode.x()];
        let vy = self.regs[self.opcode.y()];

        self.regs[0xF] = 0;
        if vx > vy {
            self.regs[0xF] = 1;
        }
        self.regs[self.opcode.x()] = vx.wrapping_sub(vy);

        self.pc += 2;
    }
    fn shift_and_rotate_vx_right(&mut self) {
        let x = self.opcode.x();
        self.regs[0xF] = self.regs[x] & 1;
        self.regs[x] = self.regs[x] >> 1;
        self.pc += 2;
    }
    fn decrement_vy_by_vx_no_borrow(&mut self) {
        let x = self.opcode.x();
        let vx = self.regs[x];
        let vy = self.regs[self.opcode.y()];

        self.regs[0xF] = 0;
        if vy > vx {
            self.regs[0xF] = 1;
        }
        self.regs[x] = vy.wrapping_sub(vx);

        self.pc += 2;
    }
    fn shift_and_rotate_vx_left(&mut self) {
        let x = self.opcode.x();
        self.regs[0xF] = self.regs[x] >> 7;
        self.regs[x] = self.regs[x].wrapping_add(self.regs[x]);
        self.pc += 2;
    }
    fn skip_instr_if_vx_not_vy(&mut self) {
        println!("Not Implemented.");
        self.pc += 4; // TODO: Change this
    }
    fn set_index_register_to_pl(&mut self) {
        self.index = self.opcode.xyz();
        self.pc += 2;
    }
    fn jump_to_v0_plus_pl(&mut self) {
        println!("Not Implemented.");
        self.pc += 2;
    }
    fn set_vx_rand_byte_and_pl(&mut self) {
        let random: u8 = rand::random();
        self.regs[self.opcode.x()] = (self.opcode.yz() as u8) & random;
        self.pc += 2;
    }
    fn display_sprite_set_vf_collision(&mut self) {
        let x = self.regs[self.opcode.x()];
        let y = self.regs[self.opcode.y()];

        let mut flag = false;
        for i in 0..self.opcode.z() {
            let byte = self.mem[self.index as usize + i as usize];
            let res = self.device.write_byte(byte, x as usize, y as usize + i as usize);
            flag = flag || res;
        };

        match flag {
            true => self.regs[0xF] = 1,
            false => self.regs[0xF] = 0,
        }

        self.device.draw();
        self.pc += 2;
    }
    fn skip_instr_if_vx_pressed(&mut self) {
        println!("Not Implemented.");
        self.pc += 2;
    }
    fn skip_instr_if_vx_not_pressed(&mut self) {
        let vx = self.regs[self.opcode.x()];
        if !self.device.keyboard.check_value_pressed(vx) {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn set_vx_to_delay_timer_val(&mut self) {
        self.regs[self.opcode.x()] = self.delay_timer.get_delay();
        self.pc += 2;
    }
    fn wait_for_key_and_store_in_vx(&mut self) {
        match self.device.keyboard.get_pressed_key() {
            Some(value) => {
                self.regs[self.opcode.x()] = value;
                self.device.keyboard.reset();
                self.pc += 2;
            }
            None => {}
        }

    }
    fn set_delay_timer_to_vx(&mut self) {
        let vx = self.regs[self.opcode.x()];
        self.delay_timer.set_delay(vx);
        self.pc += 2;
    }
    fn set_sound_timer_to_vx(&mut self) {
        let vx = self.regs[self.opcode.x()];
        self.sound_timer.set_delay(vx);
        self.pc += 2;
    }
    fn increment_index_register_by_vx(&mut self) {
        self.index += self.regs[self.opcode.x()] as u16;
        self.pc += 2;
    }
    fn set_index_register_to_vx_sprite(&mut self) {
        let vx = self.regs[self.opcode.x()];
        self.index = (vx * 5) as u16;
        self.pc += 2;
    }
    fn store_bcd_of_vx_3bytes(&mut self) {
        let vx = self.regs[self.opcode.x()];
        let hunds = vx / 100 * 100;
        let tens = (vx - hunds) / 10 * 10;
        let ones = vx - hunds - tens;
        self.mem[self.index as usize] = hunds / 100;
        self.mem[self.index as usize + 1] = tens / 10;
        self.mem[self.index as usize + 2] = ones;

        self.pc += 2;
    }
    fn store_registers_through_vx(&mut self) {
        let x = self.opcode.x();

        for i in 0..x + 1 {
            let ii = i as usize;
            self.mem[self.index as usize + ii as usize] = self.regs[ii];
        }
        self.index = self.index + x as u16 - 1 as u16;
        self.pc += 2;
    }
    fn read_registers_through_vx(&mut self) {
        let x = self.opcode.x();
        let index = self.index as usize;

        for i in 0..x + 1 {
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
