use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkState {
    Offline,
    Connecting { attempt: usize },
    Online,
    Waiting { attempt: usize, at: Instant },
    Reconnecting { attempt: usize },
}
