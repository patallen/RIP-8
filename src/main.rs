mod memory;

use memory::{CPU, parse_opcode};


fn main() {
	let mut cpu = CPU::new();
	cpu.load_rom("/Users/patallen/Code/Rust/chip8/src/roms/connect4.ch8");

	// for i in 0..1000 {
	// 	println!("{}: {}", i, cpu.mem[i]);
	// }

	println!("{}", cpu.opcode_at_address(512));
	parse_opcode(4634);
}