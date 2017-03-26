extern crate sdl2;

use self::sdl2::EventPump;
use self::sdl2::keyboard::{self, Keycode};
use self::sdl2::event::Event::{self, KeyUp, KeyDown, Quit};


#[derive(Debug)]
pub struct Keyboard {
    pub key_0: bool,
    pub key_1: bool, 
    pub key_2: bool, 
    pub key_3: bool, 
    pub key_4: bool, 
    pub key_5: bool, 
    pub key_6: bool, 
    pub key_7: bool, 
    pub key_8: bool, 
    pub key_9: bool, 
    pub key_a: bool, 
    pub key_b: bool, 
    pub key_c: bool, 
    pub key_d: bool, 
    pub key_e: bool, 
    pub key_f: bool,
    pub escape: bool,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key_0: false,
            key_1: false, 
            key_2: false, 
            key_3: false, 
            key_4: false, 
            key_5: false, 
            key_6: false, 
            key_7: false, 
            key_8: false, 
            key_9: false, 
            key_a: false, 
            key_b: false, 
            key_c: false, 
            key_d: false, 
            key_e: false, 
            key_f: false,
            escape: false,
        }
    }
    pub fn handle_event(&mut self, event: sdl2::event::Event) {
        match event {
            Event::KeyDown { keycode, .. } => match keycode {
                Some(Keycode::Num1)   => self.key_1 = true,
                Some(Keycode::Num2)   => self.key_2 = true,
                Some(Keycode::Num3)   => self.key_3 = true,
                Some(Keycode::Num4)   => self.key_c = true,
                Some(Keycode::Q)      => self.key_4 = true,
                Some(Keycode::W)      => self.key_5 = true,
                Some(Keycode::E)      => self.key_6 = true,
                Some(Keycode::R)      => self.key_d = true,
                Some(Keycode::A)      => self.key_7 = true,
                Some(Keycode::S)      => self.key_8 = true,
                Some(Keycode::D)      => self.key_9 = true,
                Some(Keycode::F)      => self.key_e = true,
                Some(Keycode::Z)      => self.key_a = true,
                Some(Keycode::X)      => self.key_0 = true,
                Some(Keycode::C)      => self.key_f = true,
                Some(Keycode::V)      => self.key_b = true,
                Some(Keycode::Escape) => self.escape = true,
                _ => {},
            },
            Event::KeyUp { keycode, .. } => match keycode {
                Some(Keycode::Num1)   => self.key_1 = false,
                Some(Keycode::Num2)   => self.key_2 = false,
                Some(Keycode::Num3)   => self.key_3 = false,
                Some(Keycode::Num4)   => self.key_c = false,
                Some(Keycode::Q)      => self.key_4 = false,
                Some(Keycode::W)      => self.key_5 = false,
                Some(Keycode::E)      => self.key_6 = false,
                Some(Keycode::R)      => self.key_d = false,
                Some(Keycode::A)      => self.key_7 = false,
                Some(Keycode::S)      => self.key_8 = false,
                Some(Keycode::D)      => self.key_9 = false,
                Some(Keycode::F)      => self.key_e = false,
                Some(Keycode::Z)      => self.key_a = false,
                Some(Keycode::X)      => self.key_0 = false,
                Some(Keycode::C)      => self.key_f = false,
                Some(Keycode::V)      => self.key_b = false,
                Some(Keycode::Escape) => self.escape = false,
                _ => {},
            },
            _ => {}
        }
        println!("{:?}", self);
    }

}