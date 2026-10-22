use super::PlayoutBuffer;
use crate::PLAYOUT_TARGET;

impl PlayoutBuffer {
    pub(super) fn skip_to_target(&mut self) {
        let excess = (self.level() - PLAYOUT_TARGET as f64).floor().max(0.0) as usize;
        self.cursor += excess;
        self.stats.dropped_frames += excess as u64;
        self.trim_history();
        self.smoothed_level = self.level();
    }
}
