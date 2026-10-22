use std::time::Instant;

use ard_media::parse_message1;
use log::info;

use super::{MediaPhase, MediaSession};
use crate::{FrameDecoder, MediaFailure};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn handle_message1(&mut self, payload: &[u8], now: Instant) {
        if !matches!(self.phase, MediaPhase::AwaitingMessage1 { .. }) {
            return;
        }
        match parse_message1(payload) {
            Ok(message1) => {
                info!(
                    "[MEDIA] the Mac ({}) offers High Performance",
                    message1.model
                );
                if let Some(offer) = self.offer(now) {
                    self.phase = MediaPhase::Offering(offer);
                }
            }
            Err(error) => self.fail(MediaFailure::BadMessage1(error.to_string())),
        }
    }
}
