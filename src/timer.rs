use std::time::{Duration, Instant};

/// Timer manages time-related logic for questions (Single Responsibility Principle)
#[derive(Debug)]
pub struct Timer {
    started: Instant,
    limit: Duration,
}

impl Timer {
    pub fn new(limit_secs: u64) -> Self {
        Self {
            started: Instant::now(),
            limit: Duration::from_secs(limit_secs),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.started.elapsed()
    }

    pub fn remaining(&self) -> Duration {
        self.limit.saturating_sub(self.elapsed())
    }

    pub fn is_expired(&self) -> bool {
        self.elapsed() >= self.limit
    }

    pub fn reset(&mut self, limit_secs: u64) {
        self.started = Instant::now();
        self.limit = Duration::from_secs(limit_secs);
    }
}
