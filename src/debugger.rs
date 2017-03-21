use std::io;

use cpu::CPU;

pub struct Debugger {
	cpu: CPU,
}

impl Debugger {
	pub fn new(filepath: &str) -> Debugger {
		let mut bugger = Debugger {
			cpu: CPU::new()
		};
		bugger.cpu.load_rom(filepath);
		bugger
	}
	pub fn run(&mut self) {
		loop {
			let opcode = self.cpu.opcode_at_address(self.cpu.pc as usize);
			let pc = self.cpu.pc;
			let sp = self.cpu.sp;
			let spv = self.cpu.stack[self.cpu.sp as usize];
			print!("\n");
			print!("Code: 0x{:X}. PC: 0x{:X}. SP: 0x{:X}. *SP: 0x{:X}.\r", opcode, pc, sp, spv);

			self.cpu.cycle();

			let mut s = String::new();
			io::stdin().read_line(&mut s).unwrap();
		}
	}
}