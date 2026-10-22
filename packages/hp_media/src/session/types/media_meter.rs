use std::time::Instant;

pub(crate) struct MediaMeter {
    pub(crate) window_start: Instant,
    pub(crate) bytes: u64,
    pub(crate) pictures: u64,
}
