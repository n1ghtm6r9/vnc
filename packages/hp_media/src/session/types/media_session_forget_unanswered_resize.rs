use std::time::Instant;

use log::warn;

use super::{MediaPhase, MediaSession};
use crate::FrameDecoder;

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn forget_unanswered_resize(&mut self, now: Instant) {
        let MediaPhase::Streaming { reoffer, .. } = &mut self.phase else {
            return;
        };
        if reoffer
            .as_ref()
            .is_some_and(|offer| offer.resize && now >= offer.deadline)
        {
            *reoffer = None;
            warn!("[MEDIA] the Mac did not answer the new display, the stream it has goes on");
        }
    }
}
