use std::time::Instant;

use log::info;

use super::{MediaPhase, MediaSession};
use crate::{FrameDecoder, MediaEvent};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn send_wanted_display(&mut self, now: Instant) {
        if !matches!(self.phase, MediaPhase::Streaming { reoffer: None, .. }) {
            return;
        }
        let Some(display) = self.wanted_display.take() else {
            return;
        };
        info!("[MEDIA] asking the Mac for a stream of the new display");
        self.effects.events.push(MediaEvent::Display(display));
        if let Some(mut offer) = self.offer(now)
            && let MediaPhase::Streaming { reoffer, .. } = &mut self.phase
        {
            offer.resize = true;
            *reoffer = Some(offer);
        }
    }
}
