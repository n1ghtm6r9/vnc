#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ReceiverReport {
    pub ssrc: u32,
    pub source_ssrc: u32,
    pub highest_seq: u32,
    pub jitter: u32,
    pub last_sender_report: u32,
    pub delay_since_sender_report: u32,
    pub fraction_lost: u8,
    pub cumulative_lost: u32,
}
