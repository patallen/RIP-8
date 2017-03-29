mod view;
mod history;


use std::io::prelude::*;
use cpu::CPU;
use self::view::View;
use std::io::{Stdin, stdin};
use ::termion::input::TermRead;
use ::termion::event::{Key, Event};
use ::termion::async_stdin;
use opcodes::parse_opcode;
use self::history::LimitedFifoQueue;
use std::thread::sleep;


#[derive(PartialEq)]
pub enum Command {
	PlayToggle,
	Step,
	Back,
	Next,
	Quit,
	Reset,
	ChangeSpeed(f32)
}

#[derive(PartialEq, Debug)]
pub enum State {
	Quitting,
	Paused,
	Running,
}

pub struct Debugger<'a> {
	lines: LimitedFifoQueue<String>,
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
			lines: LimitedFifoQueue::new(200),
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
		let instr = &self.cpu.opcode.instr;
		format!("(PC:{:X}::I:{:X}) -> {:?} for 0x{:X}", pc, i, instr, self.cpu.opcode.value)
	}
	fn reset(&mut self) {
		self.cpu.reset();
		self.lines.clear();
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
		self.cpu.cycle();
		let line = self.dump_instr();
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
	fn change_speed(&mut self, delta: f32) {
		let current_hz = self.cpu.hz as f64;
		warn!("Current hz: {}. Delta: {}", current_hz, delta);

		let next = (current_hz + ((current_hz) * delta as f64)) as u32;
		self.cpu.set_speed_hz(next);
	}
	fn handle_command(&mut self) {
		match self.last_command {
			Some(Command::Next) 			=> self.step(1),
			Some(Command::Back) 			=> self.step(-1),
			Some(Command::PlayToggle) 		=> self.toggle_play(),
			Some(Command::Step) 			=> self.step(1),
			Some(Command::Reset) 			=> self.reset(),
			Some(Command::Quit) 			=> self.quit(),
			Some(Command::ChangeSpeed(val)) => self.change_speed(val),
			None => {}
		};
	}
	pub fn run(&mut self) {
		self.cpu.initialize();
		let stdin = async_stdin();
		self.view.render(&self.lines);
		let mut events = stdin.keys();
		loop {
			sleep(self.cpu.program_delay);
			self.handle_command();
			if self.state == State::Running {
				self.cycle();
			}
			if self.state == State::Quitting {
				break;
			}
			self.last_command = match events.next() {
				Some(Ok(Key::Right))		=> Some(Command::Next),
				Some(Ok(Key::Left))			=> Some(Command::Back),
				Some(Ok(Key::Char('p')))	=> Some(Command::PlayToggle),
				Some(Ok(Key::Char('n')))	=> Some(Command::Step),
				Some(Ok(Key::Char('='))) 	=> Some(Command::ChangeSpeed(0.1)),
				Some(Ok(Key::Char('-'))) 	=> Some(Command::ChangeSpeed(-0.1)),
				Some(Ok(Key::Backspace)) 	=> Some(Command::Reset),
				Some(Ok(Key::Esc))			=> Some(Command::Quit),
				_ 							=> None
			};
		}
	}
}
