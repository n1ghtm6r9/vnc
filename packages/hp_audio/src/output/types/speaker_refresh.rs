use std::sync::{Arc, Mutex};
use std::time::Instant;

use log::warn;

use super::{AudioOutput, Speaker};
use crate::{DEVICE_CHECK_INTERVAL, PlayoutBuffer};

impl Speaker {
    pub(crate) fn refresh(&mut self, buffer: &Arc<Mutex<PlayoutBuffer>>, now: Instant) {
        if self.next_check.is_some_and(|at| now < at) {
            return;
        }
        self.next_check = Some(now + DEVICE_CHECK_INTERVAL);
        if self
            .output
            .as_ref()
            .is_some_and(|output| !output.is_stale())
        {
            return;
        }
        self.output = None;
        match AudioOutput::open(buffer) {
            Ok(output) => {
                self.output = Some(output);
                self.problem = None;
            }
            Err(problem) => {
                if self.problem.as_ref() != Some(&problem) {
                    warn!("[AUDIO] the Mac's sound cannot play: {problem}");
                }
                self.problem = Some(problem);
            }
        }
    }
}
