use std::time::Instant;

use super::MediaPhase;
use crate::{MEDIA_SILENCE_LIMIT, MediaFailure, PICTURE_STALL_LIMIT};

impl MediaPhase {
    pub(crate) fn silence(&self, now: Instant) -> Option<MediaFailure> {
        let MediaPhase::Streaming {
            stream,
            reoffer: None,
        } = self
        else {
            return None;
        };
        if now.saturating_duration_since(stream.last_media_at) > MEDIA_SILENCE_LIMIT {
            return Some(MediaFailure::MediaStopped);
        }
        let waiting_since = if stream.shown {
            stream.assembler.held_since()?.max(stream.last_picture_at)
        } else {
            stream.last_picture_at
        };
        (now.saturating_duration_since(waiting_since) > PICTURE_STALL_LIMIT)
            .then_some(MediaFailure::Stalled)
    }
}
