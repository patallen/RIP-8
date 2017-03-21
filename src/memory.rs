use std::fs::File;
use std::io::Read;
use std::collections::HashMap;


pub struct CPU {
	pub mem: [u8; 4096],
	regs: [u8; 16],
	index: u16,
	stack: [u16; 16],
	opcode: u16,
}


impl CPU {
	pub fn new() -> CPU {
		CPU {
			mem: [0; 4096],
			regs: [0; 16],
			index: 0,
			stack: [0; 16],
			opcode: 0,
		}
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
		ret << 8 | ret2
	}
	pub fn parse_opcode<'a>(&mut self, code: u16) -> Result<&'a str, &'a str> {
		let hex = code & 0xF000;
		let rex = match hex {
			0x0000 => Ok(self.x0_refine_code(code)),
			0x1000 => Ok(self.x1_goto(code)),
			0x2000 => Ok(self.x2_call_sub(code)),
			0x3000 => Ok(self.x3_skip_if_eq(code)),
			0x4000 => Ok(self.x4_skip_if_neq(code)),
			0x5000 => Ok(self.x5_skip_if_regs_eq(code)),
			0x6000 => Ok(self.x6_refine_code(code)),
			0x7000 => Ok(self.x7_add_to_reg(code)),
			0x8000 => Ok(self.x8_refine_code(code)),
			0x9000 => Ok(self.x9_skip_if_regs_neq(code)),
			0xA000 => Ok(self.xa_store_in_i(code)),
			0xB000 => Ok(self.xb_jump_to(code)),
			0xC000 => Ok(self.xc_set_reg_random(code)),
			0xD000 => Ok(self.xd_draw_sprite(code)),
			0xE000 => Ok(self.xe_refine_code(code)),
			0xF000 => Ok(self.xf_refine_code(code)),
			_ => Err("Couldn't parse."),
		};
		println!("{}", rex.unwrap());
		rex
	}
	fn x0_refine_code(&mut self, code: u16) -> &'static str {
		// Could be 0x00E0, 0x00EE, or 0x0NNN
		match code {
			0x00EE 	=> "0x00EE",
			0x00E0 	=> "0x00E0",
			_ 		=> "0x0nnn",
		}
	}
	fn x1_goto(&mut self, code: u16) -> &'static str {
		"0x1nnn"
	}
	fn x2_call_sub(&mut self, code: u16) -> &'static str {
		"0x2nnn"
	}
	fn x3_skip_if_eq(&mut self, code: u16) -> &'static str {
		"0x3xnn"
	}
	fn x4_skip_if_neq(&mut self, code: u16) -> &'static str {
		"0x4xnn"
	}
	fn x5_skip_if_regs_eq(&mut self, code: u16) -> &'static str {
		"0x5xy0"
	}
	fn x6_refine_code(&mut self, code: u16) -> &'static str {
		"0x6xnn"
	}
	fn x7_add_to_reg(&mut self, code: u16) -> &'static str {
		"0x7xnn"
	}
	fn x8_refine_code(&mut self, code: u16) -> &'static str {
		// 0x8xy0, 0x8xy1, 0x8xy2, 0x8xy3, 0x8xy4, 0x8xy5, 0x8xy6, 0x8xy7, 0x8xyE
		match code & 0x000F {
			0x0 => "0x8xy0",
			0x1 => "0x8xy1",
			0x2 => "0x8xy2",
			0x3 => "0x8xy3",
			0x4 => "0x8xy4",
			0x5 => "0x8xy5",
			0x6 => "0x8xy6",
			0x7 => "0x8xy7",
			0xE => "0x8xyE",
			_ => "Error"
		}
	}
	fn x9_skip_if_regs_neq(&mut self, code: u16) -> &'static str {
		"0x9xy0"
	}
	fn xa_store_in_i(&mut self, code: u16) -> &'static str {
		"0xAnnn"
	}
	fn xb_jump_to(&mut self, code: u16) -> &'static str {
		"0xBnnn"
	}
	fn xc_set_reg_random(&mut self, code: u16) -> &'static str {
		"0xCxnn"
	}
	fn xd_draw_sprite(&mut self, code: u16) -> &'static str {
		"0xDxyn"
	}
	fn xe_refine_code(&mut self, code: u16) -> &'static str {
		// 0xEx9E, 0xExA1
		match code & 0x00FF {
			0x9E => "0xEx9E",
			0xA1 => "0xExA1",
			_ => "Error"
		}
	}
	fn xf_refine_code(&mut self, code: u16) -> &'static str {
		// 0xFx07, 0xFx0A, 0xFx15, 0xFx18, 0xFx1E, 0xFx29, 0xFx33, 0xFx55, 0xFx65
		match code & 0x00FF {
			0x07 => "0xFx07",
			0x0A => "0xFx0A",
			0x15 => "0xFx15",
			0x18 => "0xFx18",
			0x1E => "0xFx1E",
			0x29 => "0xFx29",
			0x33 => "0xFx33",
			0x55 => "0xFx55",
			0x65 => "0xFx65",
			_ => "Error"
		}
	}
}

#[test]
pub fn test_op_codes() {
	let mut cpu = CPU::new();
	let code_results: HashMap<u16, &str> = [
		(0x00EE, "0x00EE"),
		(0x00E0, "0x00E0"),
		(0x0000, "0x0nnn"),
		(0x1000, "0x1nnn"),
		(0x2000, "0x2nnn"),
		(0x3000, "0x3xnn"),
		(0x4000, "0x4xnn"),
		(0x5000, "0x5xy0"),
		(0x6000, "0x6xnn"),
		(0x7000, "0x7xnn"),
		(0x8FF0, "0x8xy0"),
		(0x8FF1, "0x8xy1"),
		(0x8FF2, "0x8xy2"),
		(0x8FF3, "0x8xy3"),
		(0x8FF4, "0x8xy4"),
		(0x8FF5, "0x8xy5"),
		(0x8FF6, "0x8xy6"),
		(0x8FF7, "0x8xy7"),
		(0x8FFE, "0x8xyE"),
		(0x9000, "0x9xy0"),
		(0xA000, "0xAnnn"),
		(0xB000, "0xBnnn"),
		(0xC000, "0xCxnn"),
		(0xD000, "0xDxyn"),
		(0xEF9E, "0xEx9E"),
		(0xEFA1, "0xExA1"),
		(0xFF07, "0xFx07"),
		(0xFF0A, "0xFx0A"),
		(0xFF15, "0xFx15"),
		(0xFF18, "0xFx18"),
		(0xFF1E, "0xFx1E"),
		(0xFF29, "0xFx29"),
		(0xFF33, "0xFx33"),
		(0xFF55, "0xFx55"),
		(0xFF65, "0xFx65"),
	].iter().cloned().collect();

	for (code, res) in &code_results {
		assert_eq!(*res, cpu.parse_opcode(*code).unwrap());
	}
}