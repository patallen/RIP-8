use ::termion;
use ::termion::event::{Key, Event};
use ::termion::input::TermRead;
use ::termion::{cursor, color, style, clear, terminal_size};
use ::termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, stdout, stdin, BufWriter, Result, Stdout};
use std::fmt;
use std::time::{Instant, Duration};

const OPTION_MENU: &'static str = "| Instructions <Left | Right> | State <P> (Pause/Play) | Quit <ESC> | Speed <UP | DWN> |";

pub struct View<'view> {
	stdout: BufWriter<RawTerminal<Stdout>>,
	message: Option<String>,
	menu: &'view str,
	height: u16,
	width: u16,
	paused: bool,
	menu_width: u8,
	last_instant: Instant,
	duration: Duration,
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
			menu_width: 40,
			last_instant: Instant::now(),
			duration: Duration::new(0, 16666666)
		};
		view.update();
		view
	}
	pub fn initialize(&mut self) {
		self.stdout.flush();
		self.update();
		self.paint_menu();
	}
	pub fn render<I>(&mut self, lines: I, info:Vec<String>)
		where I: IntoIterator,
			  I::Item: fmt::Display,
	{
		let now = Instant::now();
		if now.duration_since(self.last_instant) > self.duration {
			self.update();
			write!(self.stdout, "{}", cursor::Hide).unwrap();
			self.paint_lines(lines);
			self.stdout.flush();
			self.paint_info(info);
			self.last_instant = Instant::now();
		}
	}
	pub fn clear(&mut self) {
		write!(self.stdout, "{}", clear::All);
	}
	fn paint_lines<I>(&mut self, lines: I)
		where I: IntoIterator,
			  I::Item: fmt::Display,
	{
		let clear_width: usize = (self.width - (self.menu_width as u16)) as usize;
		for (idx, line)in lines.into_iter().enumerate() {
			let clear_string = blank_string(clear_width);
			if idx < self.height as usize {
				let line_height = ((self.height as u16) - idx as u16) - 1;
				write!(self.stdout,
					   "{}{}{}{}",
					   cursor::Goto(1, line_height),
					   clear_string,
					   cursor::Goto(1, line_height),
					   line).unwrap(); 
			} else {
				break;
			}
		}
	}
	fn paint_menu(&mut self) {
		write!(self.stdout, "{}{}{}{}",
			   style::Invert,
			   termion::cursor::Goto(1, self.height),
			   self.menu,
			   style::Reset).unwrap();
	}
	fn paint_info(&mut self, info: Vec<String>) {
		let x = self.width - 30;
		let y = 8;
		for (i, s) in info.into_iter().enumerate() {
			write!(self.stdout, "{}{}", cursor::Goto(x, (y + i) as u16), s);
		}
	}
	fn update(&mut self) {
		let (width, height) = terminal_size().unwrap();
		self.height = height;
		self.width = width;
	}
}


fn blank_string(size: usize) -> String {
    let spaces: String = (0..size).fold("".to_string(), |sp, _| format!("{} ", sp));
    spaces
}
