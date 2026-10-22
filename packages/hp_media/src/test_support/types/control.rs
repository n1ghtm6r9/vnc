use crate::MediaChannel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Control {
    Ack {
        don: u16,
    },
    Rate {
        pictures: u16,
        packets: u32,
    },
    Report {
        channel: MediaChannel,
        source_ssrc: u32,
        highest_seq: u32,
        last_sender_report: u32,
        delay: u32,
    },
    Other,
}
