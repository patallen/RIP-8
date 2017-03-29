use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use keyboard::Keyboard;
use display::Display;


pub struct Device<'d> {
    display: Display<'d> ,
    pub keyboard: Keyboard,
    pump: EventPump,
    pub quit: bool,
    pub debug_break: bool,
    pub debug_chunk: bool,
}


impl<'d> Device<'d> {
    pub fn new() -> Device<'d> {
        let context = ::sdl2::init().unwrap();
        let pump = context.event_pump().unwrap();

        Device {
            display: Display::new(context),
            keyboard: Keyboard::new(),
            pump: pump,
            quit: false,
            debug_break: false,
            debug_chunk: false,
        }
    }
    pub fn pump(&mut self) {
        for event in self.pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::B) => self.debug_break = !self.debug_break,
                    Some(Keycode::C) => self.debug_chunk = !self.debug_chunk,
                    Some(Keycode::Escape) => self.quit = true,
                    _ => self.keyboard.handle_event(event)
                },
                // Event::KeyUp { .. } => self.keyboard.handle_event(event),
                Event::Quit { .. } => self.quit = true,
                _ => {}
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
