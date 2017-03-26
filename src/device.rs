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
        let video = context.video().unwrap();
        let window = video.window("RIP-8 Emulator", 640, 320)
                          .position_centered().opengl()
                          .build().unwrap();
        let pump = context.event_pump().unwrap();
        let renderer = window.renderer().accelerated()
                             .build().unwrap();
        Device {
            display: Display::new(renderer),
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
}
