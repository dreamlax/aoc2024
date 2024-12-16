use std::time::Instant;

pub struct Timer {
    start: std::time::Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now()
        }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        eprintln!("Elapsed: {}µs", self.start.elapsed().as_micros());
    }
}
