use super::PlayoutBuffer;

impl PlayoutBuffer {
    pub(crate) fn level(&self) -> f64 {
        self.frames.len().saturating_sub(self.cursor) as f64 - self.phase
    }
}
