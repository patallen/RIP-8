extern crate rand;
extern crate sdl2;

use std::fs::File;
use std::io::Read;
use opcodes::{Instruction, Opcode};
use device::Device;
use utils::Timer;
use std::thread::sleep;
use std::time::Duration;
use self::rand::random;
use utils::Stack;


pub struct CPU<'cpu> {
    pub hz: u32,
    pub program_delay: Duration,
    pub mem: [u8; 4096],
    pub regs: [u8; 16],
    pub index: u16,
    pub stack: Stack,
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
            stack:  Stack::new(),
            index:  0x200,
            opcode: Opcode::from_code(0),
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
        self.stack.clear();
        self.index = 0x200;
        self.pc = 0x200;
        self.delay_timer = Timer::new(16_666_667);
        self.sound_timer = Timer::new(2_000_000);
    }
    pub fn initialize(&mut self) {
        // self.opcode = self.opcode_at_address(0x200);
        // warn!("{}", self.opcode.value);
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
        if hertz > 1 {
            self.program_delay = Duration::new(0, ((1.0 / hertz as f64) * 1000000000.0) as u32);
            self.hz = hertz;
        }
    }
    pub fn cycle(&mut self) {
        match self.sound_timer.get_delay() {
            0 => self.device.audio.pause(),
            _ => self.device.audio.resume(),
        }
        let pc = self.pc as usize;
        self.opcode = self.opcode_at_address(pc);
        self.device.pump();
        self.delay_timer.touch();
        self.sound_timer.touch();
        self.run_opcode_instruction();
    }
    pub fn load_rom(&mut self, filepath: &str) {
        let mut rom: Vec<u8> = Vec::new();
        let mut file = File::open(filepath).unwrap();
        file.read_to_end(&mut rom);

        for (i, byte) in rom.iter().enumerate() {
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
            Instruction::DecrementVxByVyNoBorrow_0x8XY5  =>  self.decrement_vx_by_vy_no_borrow(),
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
        // 0nnn - SYS addr
        warn!("0x{:04X} Not Implemented.", self.opcode.value);

        self.pc += 2;
    }
    fn return_from_sub(&mut self) {
        // 00EE - RET
        let res = self.stack.pop();
        self.pc = res;
        self.pc += 2;
    }
    fn clear_display(&mut self) {
        // 00E0 - CLS
        self.device.clear_display();
        self.pc += 2;
    }
    fn jump_to_location(&mut self) {
        // 1nnn - JP addr
        self.pc = self.opcode.xyz();
    }
    fn call_subroutine(&mut self) {
        // 2nnn - CALL addr
        let pc = self.pc;
        self.stack.push(pc);
        self.pc = self.opcode.xyz();
    }
    fn skip_instr_if_vx_eq_pl(&mut self) {
        // 3xkk - SE Vx, byte
        let vx = self.regs[self.opcode.x()];    
        if vx == self.opcode.yz() as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn skip_instr_if_vx_neq_pl(&mut self) {
        // 4xkk - SNE Vx, byte
        let vx = self.regs[self.opcode.x()];

        if vx != self.opcode.yz() as u8 {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn skip_instr_if_vx_eq_vy(&mut self) {
        // 5xy0 - SE Vx, Vy
        if self.regs[self.opcode.x()] == self.regs[self.opcode.y()] {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn set_vx_to_pl(&mut self) {
        // 6xkk - LD Vx, byte
        self.regs[self.opcode.x()] = self.opcode.yz() as u8;
        self.pc += 2;
    }
    fn increment_vx_by_pl(&mut self) {
        // 7xkk - ADD Vx, byte
        let x = self.opcode.x();
        let pl = self.opcode.yz();
        self.regs[x] = self.regs[x].wrapping_add(pl as u8);
        self.pc += 2;
    }
    fn set_vx_to_vy(&mut self) {
        // 8xy0 - LD Vx, Vy
        self.regs[self.opcode.x()] = self.regs[self.opcode.y()];
        self.pc += 2;
    }
    fn set_vx_to_vx_or_vy(&mut self) {
        // 8xy1 - OR Vx, Vy
        let x = self.opcode.x();
        let y = self.opcode.y();
        self.regs[x] |= self.regs[y];
        self.pc += 2;
    }
    fn set_vx_to_vx_and_vy(&mut self) {
        // 8xy2 - AND Vx, Vy
        let x = self.opcode.x();
        let y = self.opcode.y();
        self.regs[x] &= self.regs[y];
        self.pc += 2;
    }
    fn set_vx_to_vx_xor_vy(&mut self) {
        // 8xy3 - XOR Vx, Vy
        let x = self.opcode.x();
        let y = self.opcode.y();
        self.regs[x] ^= self.regs[y];
        self.pc += 2;
    }
    fn increment_vx_by_vy_carry(&mut self) {
        // 8xy4 - ADD Vx, Vy
        let vx = self.regs[self.opcode.x()];
        let vy = self.regs[self.opcode.y()];

        if vy > (0xFF - vx) {
            self.regs[0xF] = 1;
        } else {
            self.regs[0xF] = 0;
        }
        self.regs[self.opcode.x()] = vx.wrapping_add(vy);
        self.pc += 2;
    }
    fn decrement_vx_by_vy_no_borrow(&mut self) {
        // 8xy5 - SUB Vx, Vy
        let vx = self.regs[self.opcode.x()];
        let vy = self.regs[self.opcode.y()];

        if vx > vy {
            self.regs[0xF] = 1;
        } else {
            self.regs[0xF] = 0;
        }
        self.regs[self.opcode.x()] = vx.wrapping_sub(vy);

        self.pc += 2;
    }
    fn shift_and_rotate_vx_right(&mut self) {
        // 8xy6 - SHR Vx {, Vy}
        let x = self.opcode.x();
        self.regs[0xF] = self.regs[x] & 0b1;
        self.regs[x] = self.regs[x] >> 0b1;
        self.pc += 2;
    }
    fn decrement_vy_by_vx_no_borrow(&mut self) {
        // 8xy7 - SUBN Vx, Vy
        let x = self.opcode.x();
        let vx = self.regs[x];
        let vy = self.regs[self.opcode.y()];

        if vy > vx {
            self.regs[0xF] = 1;
        } else {
            self.regs[0xF] = 0
        }
        self.regs[x] = vy.wrapping_sub(vx);

        self.pc += 2;
    }
    fn shift_and_rotate_vx_left(&mut self) {
        // 8xyE - SHL Vx {, Vy}
        let x = self.opcode.x();
        self.regs[0xF] >>= 7;
        self.regs[x] = self.regs[x].wrapping_add(self.regs[x]);
        self.pc += 2;
    }
    fn skip_instr_if_vx_not_vy(&mut self) {
        // 9xy0 - SNE Vx, Vy
        let vx = self.regs[self.opcode.x()];
        let vy = self.regs[self.opcode.y()];

        if vx != vy { self.pc += 2 };
        self.pc += 2;
    }
    fn set_index_register_to_pl(&mut self) {
        // Annn - LD I, addr
        self.index = self.opcode.xyz();
        self.pc += 2;
    }
    fn jump_to_v0_plus_pl(&mut self) {
        // Bnnn - JP V0, addr
        let v0 = self.regs[0] as u16;
        let nnn = self.opcode.xyz();
        self.pc = nnn.wrapping_add(v0);
    }
    fn set_vx_rand_byte_and_pl(&mut self) {
        // Cxkk - RND Vx, byte
        let x = self.opcode.x();
        self.regs[x] = random::<u8>() & self.opcode.yz() as u8;
        self.pc += 2;
    }
    fn display_sprite_set_vf_collision(&mut self) {
        // Dxyn - DRW Vx, Vy, nibble
        let x = self.regs[self.opcode.x()] as usize;
        let y = self.regs[self.opcode.y()] as usize;
        let z = self.opcode.z();

        self.regs[0xF] = 0;
        let mut new: Vec<u8> = Vec::new();
        
        for i in 0..z {
            new.push(self.mem[i + self.index as usize]);
        }

        self.regs[0xf] = self.device.write_bytes(new, x, y);
        self.device.draw();
        self.pc += 2;
    }
    fn skip_instr_if_vx_pressed(&mut self) {
        // Ex9E - SKP Vx
        let vx = self.regs[self.opcode.x()];
        if self.device.keyboard.check_value_pressed(vx) {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn skip_instr_if_vx_not_pressed(&mut self) {
        // ExA1 - SKNP Vx
        let vx = self.regs[self.opcode.x()];
        if !self.device.keyboard.check_value_pressed(vx) {
            self.pc += 2;
        }
        self.pc += 2;
    }
    fn set_vx_to_delay_timer_val(&mut self) {
        // Fx07 - LD Vx, DT
        self.regs[self.opcode.x()] = self.delay_timer.get_delay();
        self.pc += 2;
    }
    fn wait_for_key_and_store_in_vx(&mut self) {
        // Fx0A - LD Vx, K
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
        // Fx15 - LD DT, Vx
        let vx = self.regs[self.opcode.x()];
        self.delay_timer.set_delay(vx);
        self.pc += 2;
    }
    fn set_sound_timer_to_vx(&mut self) {
        // Fx18 - LD ST, Vx
        let vx = self.regs[self.opcode.x()];
        self.sound_timer.set_delay(vx);
        self.pc += 2;
    }
    fn increment_index_register_by_vx(&mut self) {
        // Fx1E - ADD I, Vx
        let x = self.opcode.x();
        let r: u32 = self.index as u32 + self.regs[x] as u32;
        self.index = (r & 0xFFF) as u16;
        self.regs[0xf] = (r > 0xFFF) as u8;
        self.pc += 2;
    }
    fn set_index_register_to_vx_sprite(&mut self) {
        // Fx29 - LD F, Vx
        let vx = self.regs[self.opcode.x()];
        self.index = (vx * 5) as u16;
        self.pc += 2;
    }
    fn store_bcd_of_vx_3bytes(&mut self) {
        // Fx33 - LD B, Vx
        let vx = self.regs[self.opcode.x()];
        self.mem[self.index as usize] = vx / 100;
        self.mem[self.index as usize + 1] = (vx % 100) / 10;
        self.mem[self.index as usize + 2] = vx % 10;

        self.pc += 2;
    }
    fn store_registers_through_vx(&mut self) {
        // Fx55 - LD [I], Vx
        let x = self.opcode.x();

        for i in 0..(x + 1) {
            self.mem[self.index as usize] = self.regs[i as usize];
            self.index += 1;
        }
        self.pc += 2;
    }
    fn read_registers_through_vx(&mut self) {
        // Fx65 - LD Vx, [I]
        let x = self.opcode.x();

        for i in 0..(x + 1) {
            self.regs[i] = self.mem[self.index as usize];
            self.index += 1;
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
