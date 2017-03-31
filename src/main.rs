#[macro_use]
extern crate log;
extern crate log4rs;
extern crate termion;
extern crate sdl2;

mod cpu;
mod device;
mod display;
mod keyboard;
mod opcodes;
mod utils;
mod debugger;

use debugger::Debugger;
use std::env;
use std::path::Path;
use std::time::Duration;
pub const DEBUG: bool = true;
pub const DEBUG_CHUNK: u16 = 8;
pub const DO_CHUNK_DEBUG: bool = false;

fn main() {
	// use device::Beep;
	// use sdl2::audio::AudioSpecDesired;
	// let sdl_context = sdl2::init().unwrap();
	// let audio_subsystem = sdl_context.audio().unwrap();

	// let desired_spec = AudioSpecDesired {
	// 	freq: Some(44100),
	// 	channels: Some(1),
	// 	samples: Some(288),
	// };

	// let mut beep = audio_subsystem.open_playback(None, &desired_spec, | spec | {
	// 	Beep {
	// 	}
	// }).unwrap();

	// beep.resume();
	// std::thread::sleep(Duration::from_millis(2000));


    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let path = Path::new("./src/roms/");
    let rom = env::args().nth(1).unwrap() + ".ch8";
    let rom_path = path.join(rom);

    let mut debugger = cpu::CPU::new();
    let rommy = &*rom_path.to_string_lossy();

    debugger.load_rom(rommy);
    debugger.run();
}