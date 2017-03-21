mod memory;

use memory::CPU;


fn main() {
	let mut cpu = CPU::new();
	cpu.load_rom("/Users/patallen/Code/Rust/chip8/src/roms/connect4.ch8");
}