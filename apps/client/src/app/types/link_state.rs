use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    Offline,
    Connecting,
    Online,
    Waiting { attempt: usize, at: Instant },
    Reconnecting { attempt: usize },
}
