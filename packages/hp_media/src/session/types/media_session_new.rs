use std::time::Instant;

use ard_media::{MediaError, MediaSecrets};

use super::{MediaEffects, MediaMeter, MediaPhase, MediaSession};
use crate::{FrameDecoder, MESSAGE1_TIMEOUT};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn new(
        decoder: D,
        new_secrets: fn() -> Result<MediaSecrets, MediaError>,
        now: Instant,
    ) -> MediaSession<D> {
        MediaSession {
            decoder,
            new_secrets,
            phase: MediaPhase::AwaitingMessage1 {
                deadline: now + MESSAGE1_TIMEOUT,
            },
            recoveries: Vec::new(),
            pictures: 0,
            video_packets: 0,
            next_rate_control: now,
            next_report: now,
            meter: MediaMeter::new(now),
            assembled: Vec::new(),
            effects: MediaEffects::default(),
            wanted_display: None,
        }
    }
}
