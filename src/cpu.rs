use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


pub struct CPU {
	pub mem: [u8; 4096],
	regs: [u8; 16],
	index: u16,
	stack: [u16; 16],
	sp: u8,
	opcode: u16,
	pc: u16,
}


impl CPU {
    pub fn new() -> CPU {
        CPU {
            mem: [0; 4096],
            regs: [0; 16],
            index: 0,
            stack: [0; 16],
            sp: 0, // Pointer to the topmost of the stack
            opcode: 0,
            pc: 0x200
        }
    }
    pub fn cycle(&mut self) {
        let pim = self.pc as usize;
        let opcode = self.opcode_at_address(pim).to_owned();
        self.pc += 2;
        self.run_operation_for_opcode(opcode);
        self.opcode = opcode;

        // println!("Stack Pointer: {}", self.sp);
        // println!("Top of stack: {}", self.stack[self.sp as usize]);
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
    pub fn run_operation_for_opcode<'a>(&mut self, code: u16) {
        let rex = match code & 0xF000 {
            0x0000 => self.x0_refine_code(code),
            0x1000 => self.x1_goto(code),
            0x2000 => self.x2_call_sub(code),
            0x3000 => self.x3_skip_if_eq(code),
            0x4000 => self.x4_skip_if_neq(code),
            0x5000 => self.x5_skip_if_regs_eq(code),
            0x6000 => self.x6_refine_code(code),
            0x7000 => self.x7_add_to_reg(code),
            0x8000 => self.x8_refine_code(code),
            0x9000 => self.x9_skip_if_regs_neq(code),
            0xA000 => self.xa_store_in_i(code),
            0xB000 => self.xb_jump_to(code),
            0xC000 => self.xc_set_reg_random(code),
            0xD000 => self.xd_draw_sprite(code),
            0xE000 => self.xe_refine_code(code),
            0xF000 => self.xf_refine_code(code),
            _ => panic!("run_operation failed."),
        };
    }
    fn x0_refine_code(&mut self, code: u16) {
        // Could be 0x00E0, 0x00EE, or 0x0NNN
        match code {
            0x00EE  => self.x0_return_from_sub(code),
            // 0x00E0   => "0x00E0",
            // _        => "0x0nnn",
            _ => panic!("Nope")
        }
    }
    fn x0_return_from_sub(&mut self, code: u16) {
        self.pc = self.stack[self.sp as usize];
        self.sp -=1;
    }
    fn x1_goto(&mut self, code: u16) {
        // GOTO -> Set the PC to the specified address.
        // Doing so will make the interpreter pick up at this address
        // on the next cycle.
        self.pc = code & 0x0FFF;
    }
    fn x2_call_sub(&mut self, code: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = code & 0x0FFF;
    }
    fn x3_skip_if_eq(&mut self, code: u16) {
        // "0x3xnn"
    }
    fn x4_skip_if_neq(&mut self, code: u16) {
        // "0x4xnn"
    }
    fn x5_skip_if_regs_eq(&mut self, code: u16) {
        // "0x5xy0"
    }
    fn x6_refine_code(&mut self, code: u16) {
        // "0x6xnn"
    }
    fn x7_add_to_reg(&mut self, code: u16) {
        // "0x7xnn"
    }
    fn x8_refine_code(&mut self, code: u16) {
        // 0x8xy0, 0x8xy1, 0x8xy2, 0x8xy3, 0x8xy4, 0x8xy5, 0x8xy6, 0x8xy7, 0x8xyE
        // match code & 0x000F {
        //  0x0 => "0x8xy0",
        //  0x1 => "0x8xy1",
        //  0x2 => "0x8xy2",
        //  0x3 => "0x8xy3",
        //  0x4 => "0x8xy4",
        //  0x5 => "0x8xy5",
        //  0x6 => "0x8xy6",
        //  0x7 => "0x8xy7",
        //  0xE => "0x8xyE",
        //  _ => "Error"
        // }
    }
    fn x9_skip_if_regs_neq(&mut self, code: u16) {
        // "0x9xy0"
    }
    fn xa_store_in_i(&mut self, code: u16) {
        // "0xAnnn"
    }
    fn xb_jump_to(&mut self, code: u16) {
        // "0xBnnn"
    }
    fn xc_set_reg_random(&mut self, code: u16) {
        // "0xCxnn"
    }
    fn xd_draw_sprite(&mut self, code: u16) {
        // "0xDxyn"
    }
    fn xe_refine_code(&mut self, code: u16) {
        // // 0xEx9E, 0xExA1
        // match code & 0x00FF {
        //  0x9E => "0xEx9E",
        //  0xA1 => "0xExA1",
        //  _ => "Error"
        // }
    }
    fn xf_refine_code(&mut self, code: u16) {
        // // 0xFx07, 0xFx0A, 0xFx15, 0xFx18, 0xFx1E, 0xFx29, 0xFx33, 0xFx55, 0xFx65
        // match code & 0x00FF {
        //  0x07 => "0xFx07",
        //  0x0A => "0xFx0A",
        //  0x15 => "0xFx15",
        //  0x18 => "0xFx18",
        //  0x1E => "0xFx1E",
        //  0x29 => "0xFx29",
        //  0x33 => "0xFx33",
        //  0x55 => "0xFx55",
        //  0x65 => "0xFx65",
        //  _ => "Error"
        // }
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

pub fn parse_opcode(code: u16) -> Result<u16, &'static str> {

    match code & 0xF000 {
        0x0000 =>
            match code & 0x00FF {
                0xE0 => Ok(0x00E0),
                0xEE => Ok(0x00EE),
                _    => Ok(0x0000)
            },
        0x0000 => Ok(0x0000),
        0x1000 => Ok(0x1000),
        0x2000 => Ok(0x2000),
        0x3000 => Ok(0x3000),
        0x4000 => Ok(0x4000),
        0x5000 => Ok(0x5000),
        0x6000 => Ok(0x6000),
        0x7000 => Ok(0x7000),
        0x8000 =>
            match code & 0x000F {
                0x0 => Ok(0x8000),
                0x1 => Ok(0x8001),
                0x2 => Ok(0x8002),
                0x3 => Ok(0x8003),
                0x4 => Ok(0x8004),
                0x5 => Ok(0x8005),
                0x6 => Ok(0x8006),
                0x7 => Ok(0x8007),
                0xE => Ok(0x800E),
                _ => Err("Failed to get opcode at 0x8***"),
            },
        0x9000 => Ok(0x9000),
        0xA000 => Ok(0xA000),
        0xB000 => Ok(0xB000),
        0xC000 => Ok(0xC000),
        0xD000 => Ok(0xD000),
        0xE000 => 
            match code & 0x00FF {
                0x9E => Ok(0xE09E),
                0xA1 => Ok(0xE0A1),
                _ => Err("Failed to get opcode at 0xE***"),
            },
        0xF000 => 
            match code & 0x00FF {
                0x07 => Ok(0xF007),
                0x0A => Ok(0xF00A),
                0x15 => Ok(0xF015),
                0x18 => Ok(0xF018),
                0x1E => Ok(0xF01E),
                0x29 => Ok(0xF029),
                0x33 => Ok(0xF033),
                0x55 => Ok(0xF055),
                0x65 => Ok(0xF065),
                _ => Err("Failed to get opcode at 0xF***"),
            },
        _ => Err("Could not get opcode."),
    }
}

#[test]
pub fn test_parse_opcode() {
    let mut cpu = CPU::new();
    let code_results: HashMap<u16, u16> = [
        (0x00EE, 0x00EE),
        (0x00E0, 0x00E0),
        (0x0000, 0x0000),
        (0x1000, 0x1000),
        (0x2000, 0x2000),
        (0x3000, 0x3000),
        (0x4000, 0x4000),
        (0x5000, 0x5000),
        (0x6000, 0x6000),
        (0x7000, 0x7000),
        (0x8FF0, 0x8000),
        (0x8FF1, 0x8001),
        (0x8FF2, 0x8002),
        (0x8FF3, 0x8003),
        (0x8FF4, 0x8004),
        (0x8FF5, 0x8005),
        (0x8FF6, 0x8006),
        (0x8FF7, 0x8007),
        (0x8FFE, 0x800E),
        (0x9000, 0x9000),
        (0xA000, 0xA000),
        (0xB000, 0xB000),
        (0xC000, 0xC000),
        (0xD000, 0xD000),
        (0xEF9E, 0xE09E),
        (0xEFA1, 0xE0A1),
        (0xFF07, 0xF007),
        (0xFF0A, 0xF00A),
        (0xFF15, 0xF015),
        (0xFF18, 0xF018),
        (0xFF1E, 0xF01E),
        (0xFF29, 0xF029),
        (0xFF33, 0xF033),
        (0xFF55, 0xF055),
        (0xFF65, 0xF065),
    ].iter().cloned().collect();

    for (code, res) in &code_results {
        let result = parse_opcode(*code).unwrap();
        assert_eq!(*res, parse_opcode(*code).unwrap());
    }
}