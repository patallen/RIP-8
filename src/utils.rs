use std::time::{Instant, Duration};



pub struct Timer {
    last_instant: Option<Instant>,
    delay: u8,
    duration: u32,
}

impl Timer {
    pub fn new(duration: u32) -> Timer {
        Timer {
            last_instant: None,
            delay: 0,
            duration: duration,
        }
    }
    pub fn touch(&mut self) {
        match self.last_instant {
            None => {},
            Some(value) => {
                let now = Instant::now();
                if now.duration_since(value) > Duration::new(0, self.duration) {
                    self.delay -= 1;
                    match self.delay {
                        0 => self.last_instant = None,
                        _ => self.last_instant = Some(now),
                    }
                }
            }
            
        }
    }
    pub fn set_delay(&mut self, delay: u8) {
        self.delay = delay;
        self.last_instant = Some(Instant::now());
    }
    pub fn get_delay(&mut self) -> u8 {
        self.delay
    }
}


pub struct Stack {
    stack: [u16; 16],
    index: Option<usize>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            stack: [0; 16],
            index: None,
        }
    }
    pub fn current_index(&self) -> Option<usize> {
        self.index
    }
    fn push(&mut self, value: u16) {
        self.increment_index();
        self.stack[self.index.unwrap()] = value;
    }
    fn pop(&mut self) -> u16 {
        let rv = self.stack[self.index.unwrap()];
        self.stack[self.index.unwrap()] = 0;
        self.decrement_index();
        rv
    }
    fn increment_index(&mut self) {
        match self.index {
            Some(15) => { panic!("Stack Index HOOB") },
            Some(val) => { self.index = Some(val + 1) },
            None => { self.index = Some(1) },
        }
    }
    fn decrement_index(&mut self) {
        match self.index {
            Some(0) => {self.index = None },
            Some(val) => { self.index = Some(val - 1) },
            None => panic!("Stack Index LOOB"),
        }
    }
}