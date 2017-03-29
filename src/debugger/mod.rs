mod view;
mod history;


use std::io::prelude::*;
use cpu::CPU;
use self::view::View;
use std::io::{Stdin, stdin};
use ::termion::input::TermRead;
use ::termion::event::{Key, Event};
use ::termion::async_stdin;
use opcodes::{parse_opcode, Instruction, Opcode};
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
		let disassembled = self.disassemble_opcode(&self.cpu.opcode, &self.cpu);
		format!("(PC:{:03X}|I:{:03X})::0x{:04X} -> {}", pc, i, self.cpu.opcode.value, disassembled)
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
		self.render();
	}
	fn render(&mut self) {
		let line = self.dump_instr();
		let strings = vec![
			format!(
					"     Clock Speed: {}hz", self.cpu.hz),
					"----------------------------".to_owned(),
					"    Regs    |     Stack     ".to_owned(),
					"------------|---------------".to_owned(),
			format!("v[0] = 0x{:02X} | s[0] = 0x{:04X}", self.cpu.regs[0], self.cpu.stack[0] as usize),
			format!("v[1] = 0x{:02X} | s[1] = 0x{:04X}", self.cpu.regs[1], self.cpu.stack[1] as usize),
			format!("v[2] = 0x{:02X} | s[2] = 0x{:04X}", self.cpu.regs[2], self.cpu.stack[2] as usize),
			format!("v[3] = 0x{:02X} | s[3] = 0x{:04X}", self.cpu.regs[3], self.cpu.stack[3] as usize),
			format!("v[4] = 0x{:02X} | s[4] = 0x{:04X}", self.cpu.regs[4], self.cpu.stack[4] as usize),
			format!("v[5] = 0x{:02X} | s[5] = 0x{:04X}", self.cpu.regs[5], self.cpu.stack[5] as usize),
			format!("v[6] = 0x{:02X} | s[6] = 0x{:04X}", self.cpu.regs[6], self.cpu.stack[6] as usize),
			format!("v[7] = 0x{:02X} | s[7] = 0x{:04X}", self.cpu.regs[7], self.cpu.stack[7] as usize),
			format!("v[8] = 0x{:02X} | s[8] = 0x{:04X}", self.cpu.regs[8], self.cpu.stack[8] as usize),
			format!("v[9] = 0x{:02X} | s[9] = 0x{:04X}", self.cpu.regs[9], self.cpu.stack[9] as usize),
			format!("v[A] = 0x{:02X} | s[A] = 0x{:04X}", self.cpu.regs[0xA], self.cpu.stack[0xA] as usize),
			format!("v[B] = 0x{:02X} | s[B] = 0x{:04X}", self.cpu.regs[0xB], self.cpu.stack[0xB] as usize),
			format!("v[C] = 0x{:02X} | s[C] = 0x{:04X}", self.cpu.regs[0xC], self.cpu.stack[0xC] as usize),
			format!("v[D] = 0x{:02X} | s[D] = 0x{:04X}", self.cpu.regs[0xD], self.cpu.stack[0xD] as usize),
			format!("v[E] = 0x{:02X} | s[E] = 0x{:04X}", self.cpu.regs[0xE], self.cpu.stack[0xE] as usize),
			format!("v[F] = 0x{:02X} | s[F] = 0x{:04X}", self.cpu.regs[0xF], self.cpu.stack[0xF] as usize),
			format!("                         "),
			format!("    PC: 0x{:X} || I: 0x{:X}   ", self.cpu.pc, self.cpu.index as usize),
		];
		self.lines.push(line);
		self.view.render(&self.lines, strings);
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
		self.view.initialize();
		self.render();
		let stdin = async_stdin();
		let mut events = stdin.keys();
		loop {
			sleep(self.cpu.program_delay);
			self.handle_command();
			if self.state == State::Running {
				self.cycle();
			}
			if self.state == State::Quitting {
				self.view.clear();
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
	pub fn disassemble_opcode(&self, opcode: &Opcode, cpu: &CPU) -> String {
	    match opcode.instr {
	        Instruction::SysAddressJump_0x0NNN          => { format!("Jump to address {xyz:03X}", xyz=opcode.xyz()) },
	        Instruction::ClearDisplay_0x00E0            => { format!("Clear the display") },
	        Instruction::RetFromSubroutine_0x00EE       => { format!("Return from sub: set pc = stack[sp] and set pc -= 1") },
	        Instruction::JumpLocation_0x1NNN            => { format!("Jump to address: set PC = 0x{xyz:03X}", xyz=opcode.xyz()) },
	        Instruction::CallSubroutine_0x2NNN          => { format!("Call subroutine: set PC = 0x{xyz:03X}, set sp += 1, set pc = {xyz:03X}", xyz=opcode.xyz()) },
	        Instruction::SkipInstrIfVxEqPL_0x3XNN       => { format!("Skip instruction if v[{x:01X}] == 0x{yz:02X}", x=opcode.x(), yz=opcode.yz()) },
	        Instruction::SkipInstrIfVxNotEqPL_0x4XNN    => { format!("Skip instruction if v[{x:01X}] != 0x{yz:02X}", x=opcode.x(), yz=opcode.yz()) },
	        Instruction::SkipInstrIfVxVy_0x5XY0         => { format!("Skip instruction if v[{x:01X}] == v[{y:01X}]", x=opcode.x(), y=opcode.y()) },
	        Instruction::SetVxToPL_0x6XNN               => { format!("Set v[{x:01X}] to 0x{yz:02X}", x=opcode.x(), yz=opcode.yz()) },
	        Instruction::IncrementVxByPL_0x7XNN         => { format!("Increment v[{x:01X}] by 0x{yz:02X}", x=opcode.x(), yz=opcode.yz()) },
	        Instruction::SetVxToVy_0x8XY0               => { format!("Set v[{x:01X}] to v[{y:01X}]", x=opcode.x(), y=opcode.y()) },
	        Instruction::SetVxToVxORVy_0x8XY1           => { format!("Set v[{x:01X}] to v[{x:01X}] | v[{y:01X}]", x=opcode.x(), y=opcode.y()) },
	        Instruction::SetVxToVxANDVy_0x8XY2          => { format!("Set v[{x:01X}] to v[{x:01X}] & v[{y:01X}]", x=opcode.x(), y=opcode.y()) },
	        Instruction::SetVxToVxXORVy_0x8XY3          => { format!("Set v[{x:01X}] to v[{x:01X}] ^ v[{y:01X}]", x=opcode.x(), y=opcode.y()) },
	        Instruction::IncrementVxByVyAndCarry_0x8XY4 => { format!("Increment v[{x:01X}] by v[{y:01X}](yy) and set v[F] = 1 if overflow", x=opcode.x(), y=opcode.y()) },
	        Instruction::DecrementVxByVyNoBorrow_0x8XY5 => { format!("Decrement v[{x:01X}] by v[{y:01X}](yy) and set v[F] = 1 if v[x] > v[{y:01X}]", x=opcode.x(), y=opcode.y()) },
	        Instruction::ShiftAndRotateVxRight_0x8XY6   => { format!("Shift and rotate v[{x:01X}] right", x=opcode.x()) },
	        Instruction::DecrementVyByVxNoBorrow_0x8XY7 => { format!("Decrement v[{y:01X}](yy) by v[{x:01X}] and set v[F] = 1 if v[{y:01X}] > v[{x:01X}]", x=opcode.x(), y=opcode.y()) },
	        Instruction::ShiftAndRotateVxLeft_0x8XYE    => { format!("Shift and rotate v[{x:01X}] left", x=opcode.x()) },
	        Instruction::SkipInstrIfVxNotVy_0x9XY0      => { format!("Skip instruction if v[{x:01X}] != v[{y:01X}]", x=opcode.x(), y=opcode.y()) },
	        Instruction::SetIndexRegToPL_0xANNN         => { format!("Set index to 0x{xyz:03X}", xyz=opcode.xyz()) },
	        Instruction::JumpToV0PlusPL_0xBNNN          => { format!("Jump to location: set pc = v[0] + 0x{xyz:03X}", xyz=opcode.xyz()) },
	        Instruction::SetVxRandByteANDPL_0xCXNN      => { format!("Set v[{x:01X}] to randbyte(0xNNN) & 0x{yz:02X}", x=opcode.x(), yz=opcode.yz()) },
	        Instruction::DisplaySpriteSetVfColl_0xDXYN  => { format!("Display {z}-byte sprite at (v[{x:01X}], v[{y:01X}]). Set v[F] = 1 if collision", x=opcode.x(), y=opcode.y(), z=opcode.z()) },
	        Instruction::SkipInstrIfVxPressed_0xEX9E    => { format!("Skip instruction if v[{x:01X}](keycode) pressed", x=opcode.x()) },
	        Instruction::SkipInstrIfVxNotPressed_0xEXA1 => { format!("Skip instruction if v[{x:01X}](keycode) not pressed", x=opcode.x()) },
	        Instruction::SetVxToDelayTimerVal_0xFX07    => { format!("Set v[{x:01X}] to value of delay timer (xxx)", x=opcode.x()) },
	        Instruction::WaitForKeyStoreInVx_0xFX0A     => { format!("Wait for key and store it's value in v[{x:01X}]", x=opcode.x()) },
	        Instruction::SetDelayTimerToVx_0xFX15       => { format!("Set delay timer to v[{x:01X}]", x=opcode.x()) },
	        Instruction::SetSoundTimerToVx_0xFX18       => { format!("Set sound timer to v[{x:01X}]", x=opcode.x()) },
	        Instruction::IncrementIndexRegByVx_0xFX1E   => { format!("Set index = index + v[{x:01X}]", x=opcode.x()) },
	        Instruction::SetIndexRegToVxSprite_0xFX29   => { format!("Set index equal to the v[{x:01X}]th sprite (v[{x:01X}] * 5)", x=opcode.x()) },
	        Instruction::StoreBCDOfVxIn3Bytes_0xFX33    => { format!("Store BCD of v[{x:01X}](xxx) in mem[i], mem[i+1], mem[i+2]", x=opcode.x()) },
	        Instruction::StoreRegsUptoVx_0xFX55         => { format!("Store v[0] through v[{x:01X}] in mem[index] through mem[index + {x:01X}]", x=opcode.x()) },
	        Instruction::ReadRegsUptoVx_0xFX65          => { format!("Store mem[index] through mem[index + {x:01X}] in v[0] through v[{x:01X}]", x=opcode.x()) },
	    }
	}
}