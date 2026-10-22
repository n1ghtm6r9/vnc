use std::time::Instant;

use super::MediaPhase;
use crate::MediaFailure;

impl MediaPhase {
    pub(crate) fn expired(&self, now: Instant) -> Option<MediaFailure> {
        match self {
            MediaPhase::AwaitingMessage1 { deadline } if now >= *deadline => {
                Some(MediaFailure::NoMessage1)
            }
            MediaPhase::Offering(offer) if now >= offer.deadline => Some(MediaFailure::NoAnswer),
            MediaPhase::Waiting { deadline, .. } if now >= *deadline => {
                Some(MediaFailure::NoPicture)
            }
            MediaPhase::Streaming {
                reoffer: Some(offer),
                ..
            } if !offer.resize && now >= offer.deadline => Some(MediaFailure::MediaStopped),
            _ => None,
        }
    }
}
