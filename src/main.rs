mod cpu;
mod device;
mod display;
mod keyboard;
mod opcodes;

use std::env;
use std::path::Path;
use cpu::CPU;

pub const DEBUG: bool = true;
pub const DEBUG_CHUNK: u16 = 8;
pub const DO_CHUNK_DEBUG: bool = false;

fn main() {
    let path = Path::new("./roms/");
    let rom = env::args().nth(1).unwrap() + ".ch8";
    let rom_path = path.join(rom);

    let mut cpu = CPU::new();
    cpu.load_rom(&*rom_path.to_string_lossy());
    cpu.run();
}