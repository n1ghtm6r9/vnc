use std::time::Instant;

use ard_media::{AssemblerEvent, MediaError, MediaSecrets};

use super::{MediaEffects, MediaMeter, MediaPhase};

pub(crate) struct MediaSession<D> {
    pub(crate) decoder: D,
    pub(crate) new_secrets: fn() -> Result<MediaSecrets, MediaError>,
    pub(crate) phase: MediaPhase,
    pub(crate) recoveries: Vec<Instant>,
    pub(crate) pictures: u64,
    pub(crate) video_packets: u32,
    pub(crate) next_rate_control: Instant,
    pub(crate) next_report: Instant,
    pub(crate) meter: MediaMeter,
    pub(crate) assembled: Vec<AssemblerEvent>,
    pub(crate) effects: MediaEffects,
    pub(crate) wanted_display: Option<Vec<u8>>,
}
