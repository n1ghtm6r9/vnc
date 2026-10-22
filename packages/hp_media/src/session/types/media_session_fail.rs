use log::{info, warn};

use super::{MediaPhase, MediaSession};
use crate::{FrameDecoder, MediaEvent, MediaFailure, MediaState};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn fail(&mut self, failure: MediaFailure) {
        if failure.needs_reconnect() {
            warn!("[MEDIA] High Performance ends: {failure}");
        } else {
            info!("[MEDIA] {failure}");
        }
        self.phase = MediaPhase::Finished;
        self.effects
            .events
            .push(MediaEvent::State(MediaState::Failed(failure)));
    }
}
