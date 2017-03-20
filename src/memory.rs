use std::fs::File;
use std::io::Read;

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
}
