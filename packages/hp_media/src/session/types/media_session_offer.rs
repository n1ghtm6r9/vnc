use std::time::Instant;

use ard_media::{build_offer, ntp_time};

use super::{MediaSession, PendingOffer};
use crate::{FrameDecoder, MediaEvent, MediaFailure, unix_time};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn offer(&mut self, now: Instant) -> Option<PendingOffer> {
        let made = (self.new_secrets)().and_then(|secrets| {
            let offer = build_offer(&secrets, ntp_time(unix_time()))?;
            Ok((secrets, offer))
        });
        match made {
            Ok((secrets, offer)) => {
                self.effects.events.push(MediaEvent::Offer(offer));
                Some(PendingOffer::new(secrets, now))
            }
            Err(error) => {
                self.fail(MediaFailure::Offer(error.to_string()));
                None
            }
        }
    }
}
