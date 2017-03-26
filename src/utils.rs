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