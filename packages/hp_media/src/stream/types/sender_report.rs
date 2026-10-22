use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SenderReport {
    pub(crate) middle: u32,
    pub(crate) at: Instant,
}
