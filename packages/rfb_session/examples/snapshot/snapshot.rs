use std::time::{Duration, Instant};

use rfb_session::SessionStats;

pub(crate) struct Snapshot {
    pub(super) started: Instant,
    pub(super) width: u16,
    pub(super) height: u16,
    pub(super) pixels: Vec<u32>,
    pub(super) first_frame: Option<Duration>,
    pub(super) stats: Option<SessionStats>,
}
