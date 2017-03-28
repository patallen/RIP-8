mod view;

use std::io::prelude::*;
use cpu::CPU;
use self::view::View;
use std::io::{Stdin, stdin};
use ::termion::input::TermRead;
use ::termion::event::{Key, Event};
use ::termion::async_stdin;
use opcodes::parse_opcode;

#[derive(PartialEq)]
pub enum Command {
	PlayToggle,
	Step,
	Back,
	Next,
	Quit,
	Reset,
}

#[derive(PartialEq, Debug)]
pub enum State {
	Quitting,
	Paused,
	Running,
}

pub struct Debugger<'a> {
	lines: Vec<String>,
	pub cpu: CPU<'a>,
	view: View<'a>,
	record: usize,
	last_command: Option<Command>,
	state: State,
	speed: u8,
}

impl<'a> Debugger<'a> {
	pub fn new() -> Debugger<'a> {
		Debugger {
			lines: Vec::new(),
			cpu: CPU::new(),
			view: View::new(),
			record: 0,
			last_command: None,
			state: State::Paused,
			speed: 100
		}
	}
	fn dump_instr(&self) -> String {
		let pc = self.cpu.pc;
		let i = self.cpu.index;
		let instr = parse_opcode(self.cpu.opcode);
		format!("(PC:{:X}::I:{:X}) -> {:?} for 0x{:X}", pc, i, instr, self.cpu.opcode)
	}
	fn reset(&mut self) {
		self.cpu.reset();
		self.lines = Vec::new();
		self.record = 0;
		self.last_command = None;
		self.state = State::Paused;
	}
	pub fn load_rom(&mut self, rom: &str) {
		self.cpu.load_rom(rom);
	}
	fn step(&mut self, distance: i32) {
		self.state = State::Paused;
		for _ in 0..distance {
			self.cycle();
		}
	}
	fn cycle(&mut self) {
		let message = format!("{:?}", self.cpu.opcode);
		let line = self.dump_instr();
		self.cpu.cycle();
		self.lines.push(line);
		self.view.render(&self.lines);
	}
	fn toggle_play(&mut self) {
		match self.state {
			State::Paused => self.state = State::Running,
			_ => self.state = State::Paused,
		}
	}
	fn quit(&mut self) {
		self.state = State::Quitting;
	}
	fn handle_command(&mut self) {
		match self.last_command {
			Some(Command::Next) => self.step(1),
			Some(Command::Back) => self.step(-1),
			Some(Command::PlayToggle) => self.toggle_play(),
			Some(Command::Step) => self.step(1),
			Some(Command::Reset) => self.reset(),
			Some(Command::Quit) => self.quit(),
			None => {}
		};
	}
	pub fn run(&mut self) {
		let stdin = async_stdin();
		self.view.render(&self.lines);
		let mut events = stdin.keys();
		loop {
			self.handle_command();
			if self.state == State::Running {
				self.cycle();
			}
			if self.state == State::Quitting {
				break;
			}
			self.last_command = match events.next() {
				Some(Ok(Key::Char('}')))	=> Some(Command::Next),
				Some(Ok(Key::Char('{')))	=> Some(Command::Back),
				Some(Ok(Key::Char('p')))	=> Some(Command::PlayToggle),
				Some(Ok(Key::Char('o')))	=> Some(Command::Step),
				Some(Ok(Key::Backspace)) 	=> Some(Command::Reset),
				Some(Ok(Key::Esc))			=> Some(Command::Quit),
				_ 							=> None
			};
		}
	}
}
