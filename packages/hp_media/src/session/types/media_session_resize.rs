use std::time::Instant;

use super::MediaSession;
use crate::FrameDecoder;

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn resize(&mut self, display: Vec<u8>, now: Instant) {
        self.wanted_display = Some(display);
        self.send_wanted_display(now);
    }
}
