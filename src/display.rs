extern crate sdl2;

use self::sdl2::render::Renderer;
use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;
use self::sdl2::video::Window;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
const DISPLAY_WIDTH: usize = SCREEN_WIDTH * 20;
const DISPLAY_HEIGHT: usize = SCREEN_HEIGHT * 20;
const PIXEL_SIZE: usize = DISPLAY_HEIGHT / SCREEN_HEIGHT;
const TITLE: &str = "RIP-8::CHIP-8";

pub struct Display<'d> {
    pixels: [u8; SCREEN_PIXELS],
    width: usize,
    height: usize,
    renderer: Renderer<'d>,
}


impl<'d> Display<'d> {
    pub fn new(context: sdl2::Sdl) -> Display<'d> {
        let video = context.video().unwrap();
        let window = video.window(TITLE, DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
                          .position_centered().opengl().build().unwrap();
        let mut renderer = window.renderer().accelerated()
                             .build().unwrap();
        Display {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            pixels: [0; SCREEN_PIXELS],
            renderer: renderer,
        }
    }
    pub fn write_byte(&mut self, byte: u8, x: usize, y: usize) -> bool {
        let bytearr = self.byte_to_digits(byte);
        let limit = (y+1) * SCREEN_WIDTH - 1;
        let start = x + y * SCREEN_WIDTH;

        let mut erased = false;
        for (i, bit) in bytearr.into_iter().enumerate() {
            if i + start > limit {
                break;
            }
            let old = self.pixels[i + start];

            self.pixels[i + start] = old ^ bit;
            if old == 1 && self.pixels[i + start] != old {
                erased = true
            }
        }
        erased
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
        self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        self.renderer.clear();
        self.renderer.set_draw_color(Color::RGB(255, 255, 255));
        for (idx, p) in self.pixels.into_iter().enumerate() {
            if *p > 0{
                let x = idx - (idx / SCREEN_WIDTH * SCREEN_WIDTH);
                let y = idx / SCREEN_WIDTH;
                let pixel = Pixel::new(x, y);
                self.renderer.fill_rect(pixel.to_sdl());
            }
        }
        self.renderer.present();
    }
    pub fn clear(&mut self) {
        self.pixels = [0; SCREEN_WIDTH * SCREEN_HEIGHT];
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