use std::time::{Duration, Instant};

pub struct Timer {
    tick: Instant,
    elapsed: Duration,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            tick: Instant::now(),
            elapsed: Duration::from_secs(0),
        }
    }
    pub fn tick(&mut self) {
        let tock = Instant::now();
        let elapsed = tock.duration_since(self.tick);

        self.tick = tock;
        self.elapsed = elapsed;
    }
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }
    pub fn fps(&self) -> f64 {
        Duration::from_secs(1).as_secs_f64() / self.elapsed.as_secs_f64()
    }
}
