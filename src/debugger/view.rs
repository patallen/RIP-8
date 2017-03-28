use ::termion;
use ::termion::event::{Key, Event};
use ::termion::input::TermRead;
use ::termion::{cursor, color, style, clear, terminal_size};
use ::termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, stdout, stdin, BufWriter, Result, Stdout};
const OPTION_MENU: &'static str = "| '}':next | '{':prev | 'P':Pause | esc:quit |";

pub struct View<'view> {
	stdout: BufWriter<RawTerminal<Stdout>>,
	message: Option<String>,
	menu: &'view str,
	height: u16,
	width: u16,
	paused: bool,
}
impl<'view> View<'view> {
	pub fn new() -> View<'view> {
		let mut view = View {
			stdout: BufWriter::new(stdout().into_raw_mode().unwrap()),
			message: None,
			menu: OPTION_MENU,
			height: 0,
			width: 0,
			paused: true,
		};
		view.update();
		view
	}
	pub fn render(&mut self, lines: &Vec<String>) {
		self.update();
		write!(self.stdout, "{}", clear::All).unwrap();
		self.paint_menu();
		self.paint_lines(lines);
		self.stdout.flush();
	}
	fn paint_lines(&mut self, lines: &Vec<String>) {
		for (idx, line)in lines.into_iter().enumerate() {
			write!(self.stdout, "{}{}", cursor::Goto(1, (idx as u16) + 1), line);
		}
	}
	fn paint_menu(&mut self) {
		write!(self.stdout, "{}{}{}{}",
			   style::Invert,
			   termion::cursor::Goto(1, self.height),
			   self.menu,
			   style::Reset);
	}
	fn update(&mut self) {
		let (width, height) = terminal_size().unwrap();
		self.height = height;
		self.width = width;
	}
}

