use std::sync::atomic::Ordering;

use super::AudioOutput;
use crate::default_output_id;

impl AudioOutput {
    pub(crate) fn is_stale(&self) -> bool {
        self.failed.load(Ordering::Relaxed) || default_output_id() != self.device
    }
}
