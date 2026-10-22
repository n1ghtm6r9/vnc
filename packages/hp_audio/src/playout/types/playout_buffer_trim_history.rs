use super::PlayoutBuffer;
use crate::KERNEL_HALF;

impl PlayoutBuffer {
    pub(super) fn trim_history(&mut self) {
        let played = (self.cursor + 1).saturating_sub(KERNEL_HALF);
        self.frames.drain(..played);
        self.cursor -= played;
    }
}
