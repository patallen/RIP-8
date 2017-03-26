extern crate sdl2;

use self::sdl2::EventPump;
use self::sdl2::event::Event::{KeyUp, KeyDown, Quit};
use self::sdl2::render::Renderer;

use keyboard::Keyboard;
use display::Display;


pub struct Device<'d> {
    display: Display<'d> ,
    keyboard: Keyboard,
    pump: EventPump,
    pub quit: bool,
}


impl<'d> Device<'d> {
    pub fn new() -> Device<'d> {
        let context = sdl2::init().unwrap();
        let pump = context.event_pump().unwrap();

        Device {
            display: Display::new(context),
            keyboard: Keyboard::new(),
            pump: pump,
            quit: false
        }
    }
    pub fn pump(&mut self) {
        for event in self.pump.poll_iter() {
            match event {
                KeyDown { .. } => self.keyboard.handle_event(event),
                KeyUp { .. } => self.keyboard.handle_event(event),
                Quit { .. } => self.quit = true,
                _ => {}
            }
            if self.keyboard.escape {
                self.quit = true
            }
        }
    }
    pub fn write_byte(&mut self, byte: u8, x: usize, y:usize) -> bool {
        self.display.write_byte(byte, x, y)
    }
    pub fn clear_display(&mut self) {
        self.display.clear();
    }
    pub fn draw(&mut self) {
        self.display.draw()
    }
}
