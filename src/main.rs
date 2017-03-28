#[macro_use]
extern crate log;
extern crate log4rs;
extern crate termion;

use log::LogLevel;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

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
use cpu::CPU;

pub const DEBUG: bool = true;
pub const DEBUG_CHUNK: u16 = 8;
pub const DO_CHUNK_DEBUG: bool = false;

fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let path = Path::new("./src/roms/");
    let rom = env::args().nth(1).unwrap() + ".ch8";
    let rom_path = path.join(rom);

    let mut debugger = Debugger::new();
    let rommy = &*rom_path.to_string_lossy();

    debugger.load_rom(rommy);
    debugger.run();
}