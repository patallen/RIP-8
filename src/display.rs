extern crate sdl2;

use self::sdl2::render::Renderer;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;


const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
const DISPLAY_WIDTH: usize = SCREEN_WIDTH * 20;
const DISPLAY_HEIGHT: usize = SCREEN_HEIGHT * 20;
const PIXEL_SIZE: usize = DISPLAY_HEIGHT / SCREEN_HEIGHT;
const TITLE: &str = "RIP-8::CHIP-8";

pub struct Display<'d> {
    pixels: [bool; SCREEN_PIXELS],
    renderer: Renderer<'d>,
}


impl<'d> Display<'d> {
    pub fn new(context: sdl2::Sdl) -> Display<'d> {
        let video = context.video().unwrap();
        let window = video.window(TITLE, DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
                          .position_centered().opengl().build().unwrap();
        let renderer = window.renderer().accelerated()
                              .build().unwrap();
        Display {
            pixels: [false; SCREEN_PIXELS],
            renderer: renderer,
        }
    }
    pub fn write_bytes(&mut self, bytes: Vec<u8>, x: usize, y: usize) -> u8 {
        let mut rv = 0;
        for (r, byte) in bytes.into_iter().enumerate() {
            let sy = (r + y) % SCREEN_HEIGHT;

            for j in 0..8 {
                let sx = (x + j) % SCREEN_WIDTH;
                let offset = sy * SCREEN_WIDTH + sx;

                let dot = &mut self.pixels[offset];
                let was_set = *dot;

                let dot_set = (byte >> (7 - j)) & 1;

                *dot = ((*dot as u8) ^ dot_set) != 0;

                rv |= (was_set && !*dot) as u8;
            }
        }
        rv
    }

    fn byte_to_digits(&self, byte: u8) -> [u8; 8] {
        let mut bytearr = [0; 8];
        for i in 0..8 {
            bytearr[i] = byte >> i & 1;
        }
        bytearr.reverse();
        bytearr
    }

    pub fn draw(&mut self) {
        self.renderer.set_draw_color(Color::RGB (28,28,28));
        self.renderer.clear();
        self.renderer.set_draw_color(Color::RGB(230, 230, 230));
        for (idx, p) in self.pixels.into_iter().enumerate() {
            if *p {
                let x = idx - (idx / SCREEN_WIDTH * SCREEN_WIDTH);
                let y = idx / SCREEN_WIDTH;
                let pixel = Pixel::new(x, y);
                self.renderer.fill_rect(pixel.to_sdl());
            }
        }
        self.renderer.present();
    }
    pub fn clear(&mut self) {
        self.pixels = [false; SCREEN_PIXELS];
    }
}

struct Pixel {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Pixel {
    pub fn new(x: usize, y: usize) -> Pixel {
        Pixel {
            x: x * PIXEL_SIZE,
            y: y * PIXEL_SIZE,
            w: PIXEL_SIZE,
            h: PIXEL_SIZE,
        }
    }
    fn to_sdl(self ) -> Rect {
        let x = self.x as i32;
        let y = self.y as i32;
        let h = self.h as u32;
        let w = self.w as u32;
        Rect::new(x, y, h, w)
    }
}

pub fn get_sub_arr(arr: &[u8; 2048], x: usize, y: usize) -> [u8; 8] {
    let start = x + (y * 64);
    let mut list: [u8; 8] = [0; 8];
    for i in 0..8 {
        list[i] = arr[i + start];
    }
    list
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
    let x = 0;
    let y = 0;
    let byte = 0b10101010;
    let mut disp = Display::new();
    let res = disp.write_byte(byte, x, y);

    let arr: [u8; 8] = get_sub_arr(&disp.pixels, x, y);
    assert_eq!(res, true);
    assert_eq!(arr, [1, 0, 1, 0, 1, 0, 1, 0])
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