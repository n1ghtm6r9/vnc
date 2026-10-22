use std::sync::atomic::Ordering;

use hp_media::{MediaEvent, MediaState};
use rfb::ENCODING_APPLE_HIGH_PERFORMANCE;

use super::MediaForwarder;
use crate::{Outgoing, SessionEvent, SessionStats};

impl MediaForwarder {
    pub(crate) fn forward(&self, event: MediaEvent) {
        match event {
            MediaEvent::Display(message) | MediaEvent::Offer(message) => {
                let _ = self.outgoing.send(Outgoing::MediaMessage(message));
            }
            MediaEvent::Stripe(stripe) => self.events.emit(SessionEvent::PictureStripe(stripe)),
            MediaEvent::State(state) => {
                let streaming = matches!(state, MediaState::Streaming { .. });
                self.streaming.store(streaming, Ordering::Relaxed);
                self.events.emit(SessionEvent::HighPerformance(state));
            }
            MediaEvent::Stats(stats) => self.events.emit(SessionEvent::Stats(SessionStats {
                bytes_per_second: stats.bytes_per_second,
                updates_per_second: stats.frames_per_second,
                last_encoding: ENCODING_APPLE_HIGH_PERFORMANCE,
            })),
        }
    }
}
