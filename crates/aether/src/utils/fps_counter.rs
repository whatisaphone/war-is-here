use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

pub struct FPSCounter {
    times: VecDeque<Instant>,
    ready: bool,
}

impl FPSCounter {
    pub fn new() -> Self {
        FPSCounter {
            times: VecDeque::new(),
            ready: false,
        }
    }

    pub fn fps(&self) -> Option<usize> {
        if !self.ready {
            return None;
        }
        Some(self.times.len())
    }

    pub fn tick(&mut self, instant: Instant) {
        self.times.push_back(instant);
        while !self.times.is_empty()
            && *self.times.front().unwrap() < instant - Duration::from_secs(1)
        {
            self.times.pop_front();
            self.ready = true;
        }
    }
}
