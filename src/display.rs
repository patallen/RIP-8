extern crate sdl2;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;


pub enum ScreenCommand {
	Clear,

}
pub struct Display {
	pixels: [u8; SCREEN_PIXELS],
	width: usize,
	height: usize,
}


impl Display {
	pub fn new() -> Display {
		Display {
			width: SCREEN_WIDTH,
			height: SCREEN_HEIGHT,
			pixels: [0; SCREEN_PIXELS]
		}
	}
	pub fn write_byte(&mut self, byte: u8, x: usize, y: usize) -> bool {
		let bytearr = self.byte_to_digits(byte);
		let limit = (y+1) * SCREEN_WIDTH - 1;
		let start = x + y * SCREEN_WIDTH;

		let mut modified = false;
		for (i, bit) in bytearr.into_iter().enumerate() {
			if i + start > limit {
				break;
			}
			let current = self.pixels[i];
			self.pixels[i + start] = current ^ bit;
			modified = modified || self.pixels[i + start] == current;
		}
		modified
	}

	fn byte_to_digits(&self, byte: u8) -> [u8; 8] {
		let mut bytearr = [0; 8];
		for i in 0..8 {
			bytearr[i] = byte >> i & 1;
		}
		bytearr.reverse();
		bytearr
	}

	pub fn draw(&self) {
		print!("\n");
		for x in 0..SCREEN_PIXELS {
			print!("{:?} ", self.pixels[x]);
			if (x + 1) % SCREEN_WIDTH == 0 {
				print!("\n");
			}
		}
	}
}

#[test]
fn test_byte_to_digits() {
	let mut disp = Display::new();
	let res = disp.byte_to_digits(0b10101010);
	let test: [u8; 8] = [1, 0, 1, 0, 1, 0, 1, 0];
	assert_eq!(test, res);
}


#[test]
fn test_write_byte() {
	let mut disp = Display::new();
	let res = disp.write_byte(0b10101010, 0, 0);
	assert_eq!(res, true);
	for (i, x) in [1, 0, 1, 0, 1, 0, 1, 0].into_iter().enumerate() {
		assert_eq!(disp.pixels[i], *x as u8);
	}
}

#[test]
fn test_write_byte_overflow() {
	let mut disp = Display::new();
	let res = disp.write_byte(0b10101010, 60, 1);
	assert_eq!(res, true);
	let mut list: [u8; 8] = [0; 8];
	let start = 60 + 64;
	for i in 0..8 {
		list[i] = disp.pixels[i + start];
	}
	assert_eq!(list, [1, 0, 1, 0, 0, 0, 0, 0])
}