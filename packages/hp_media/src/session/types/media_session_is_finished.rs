use super::{MediaPhase, MediaSession};
use crate::FrameDecoder;

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn is_finished(&self) -> bool {
        matches!(self.phase, MediaPhase::Finished)
    }
}
