use std::time::Instant;

pub(crate) struct StatsMeter {
    pub(super) window_start: Instant,
    pub(super) updates: u32,
    pub(super) bytes: u64,
}
