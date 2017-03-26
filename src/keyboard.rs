extern crate sdl2;

use self::sdl2::EventPump;
use self::sdl2::keyboard::{self, Keycode};
use self::sdl2::event::Event::{self, KeyUp, KeyDown, Quit};

#[derive(Debug)]
struct Key {
    is_pressed: bool,
    value: u8
}

impl Key {
    pub fn new(value: u8) -> Key {
        Key {
            is_pressed: false,
            value: value,
        }
    }
    pub fn reset(&mut self) {
        self.is_pressed = false
    }
    pub fn press(&mut self) {
        self.is_pressed = true
    }
    pub fn value(&self) -> u8 {
        self.value
    }
    pub fn is_pressed(&self) -> bool {
        self.is_pressed
    }
}

#[derive(Debug)]
pub struct Keyboard {
    key_0: Key,
    key_1: Key,
    key_2: Key,
    key_3: Key,
    key_4: Key,
    key_5: Key,
    key_6: Key,
    key_7: Key,
    key_8: Key,
    key_9: Key,
    key_a: Key,
    key_b: Key,
    key_c: Key,
    key_d: Key,
    key_e: Key,
    key_f: Key,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            key_0: Key::new(0x0),
            key_1: Key::new(0x1),
            key_2: Key::new(0x2),
            key_3: Key::new(0x3),
            key_4: Key::new(0x4),
            key_5: Key::new(0x5),
            key_6: Key::new(0x6),
            key_7: Key::new(0x7),
            key_8: Key::new(0x8),
            key_9: Key::new(0x9),
            key_a: Key::new(0xa),
            key_b: Key::new(0xb),
            key_c: Key::new(0xc),
            key_d: Key::new(0xd),
            key_e: Key::new(0xe),
            key_f: Key::new(0xf),
        }
    }
    pub fn handle_event(&mut self, event: sdl2::event::Event) {
        match event {
            Event::KeyDown { keycode, .. } => match keycode {
                Some(Keycode::Num1)   => self.key_1.press(),
                Some(Keycode::Num2)   => self.key_2.press(),
                Some(Keycode::Num3)   => self.key_3.press(),
                Some(Keycode::Num4)   => self.key_c.press(),
                Some(Keycode::Q)      => self.key_4.press(),
                Some(Keycode::W)      => self.key_5.press(),
                Some(Keycode::E)      => self.key_6.press(),
                Some(Keycode::R)      => self.key_d.press(),
                Some(Keycode::A)      => self.key_7.press(),
                Some(Keycode::S)      => self.key_8.press(),
                Some(Keycode::D)      => self.key_9.press(),
                Some(Keycode::F)      => self.key_e.press(),
                Some(Keycode::Z)      => self.key_a.press(),
                Some(Keycode::X)      => self.key_0.press(),
                Some(Keycode::C)      => self.key_f.press(),
                Some(Keycode::V)      => self.key_b.press(),
                _ => {},
            },
            Event::KeyUp { keycode, .. } => match keycode {
                Some(Keycode::Num1)   => self.key_1.reset(),
                Some(Keycode::Num2)   => self.key_2.reset(),
                Some(Keycode::Num3)   => self.key_3.reset(),
                Some(Keycode::Num4)   => self.key_c.reset(),
                Some(Keycode::Q)      => self.key_4.reset(),
                Some(Keycode::W)      => self.key_5.reset(),
                Some(Keycode::E)      => self.key_6.reset(),
                Some(Keycode::R)      => self.key_d.reset(),
                Some(Keycode::A)      => self.key_7.reset(),
                Some(Keycode::S)      => self.key_8.reset(),
                Some(Keycode::D)      => self.key_9.reset(),
                Some(Keycode::F)      => self.key_e.reset(),
                Some(Keycode::Z)      => self.key_a.reset(),
                Some(Keycode::X)      => self.key_0.reset(),
                Some(Keycode::C)      => self.key_f.reset(),
                Some(Keycode::V)      => self.key_b.reset(),
                _ => {},
            },
            _ => {}
        }
    }
    pub fn keys(&mut self) -> [&Key; 16] {
        [
            &self.key_0,
            &self.key_1,
            &self.key_2,
            &self.key_3,
            &self.key_4,
            &self.key_5,
            &self.key_6,
            &self.key_7,
            &self.key_8,
            &self.key_9,
            &self.key_a,
            &self.key_b,
            &self.key_c,
            &self.key_d,
            &self.key_e,
            &self.key_f,
        ]
    }
    pub fn reset(&mut self) {
        self.key_0.reset();
        self.key_1.reset();
        self.key_2.reset();
        self.key_3.reset();
        self.key_4.reset();
        self.key_5.reset();
        self.key_6.reset();
        self.key_7.reset();
        self.key_8.reset();
        self.key_9.reset();
        self.key_a.reset();
        self.key_b.reset();
        self.key_c.reset();
        self.key_d.reset();
        self.key_e.reset();
        self.key_f.reset();
    }
    pub fn key_is_pressed(&mut self) -> bool {
        self.keys().into_iter().fold(false, |l, n| l || n.is_pressed())
    }
    pub fn get_pressed_key(&mut self) -> Option<u8> {
        for key in self.keys().into_iter() {
            if key.is_pressed() {
                return Some(key.value())
            }
        }
        return None
    }
    pub fn check_value_pressed(&mut self, value: u8) -> bool {
        for key in self.keys().into_iter() {
            if key.value() == value {
                return key.is_pressed()
            }
        }
        return false
    }
}