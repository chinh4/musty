use std::{Duration, SystemTime, UNIX_EPOCH};

use amane::{Service, Text};

#[derive(Default)]
pub struct Clock {
  time: String
}

impl Service for Clock {
  pub fn new() -> Self {
    Self { time: amane::output("date +%H:%M:%S") }
  }

  fn interval() -> Duration {
    Duration::from_secs(1)
  }
  
  fn update(&mut self) -> bool {
    let time = amane::output("date +%H:%M:%S");
    let changed = time != self.time;
    self.time = time;
    changed
  }
}