use std::time::Instant;

use super::{AssemblerEvent, VideoAssembler};

impl VideoAssembler {
    pub fn tick(&mut self, now: Instant, events: &mut Vec<AssemblerEvent>) {
        self.check_stall(now, events);
    }
}
