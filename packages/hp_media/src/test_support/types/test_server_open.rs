use super::{Control, TestServer};
use crate::Datagram;

impl TestServer {
    pub fn open(&self, datagram: &Datagram) -> Control {
        let mut bytes = datagram.bytes.clone();
        let plain = self.from_client[datagram.channel.index()]
            .unprotect_rtcp(&mut bytes)
            .expect("feedback from the client opens with the client's keys");
        let word = |at: usize| {
            u32::from_be_bytes([plain[at], plain[at + 1], plain[at + 2], plain[at + 3]])
        };
        match plain[1] {
            0xc9 => Control::Report {
                channel: datagram.channel,
                source_ssrc: word(8),
                highest_seq: word(16),
                last_sender_report: word(24),
                delay: word(28),
            },
            0xcc if plain[8..12] == *b"RCTL" => Control::Rate {
                pictures: u16::from_be_bytes([plain[16], plain[17]]),
                packets: word(26),
            },
            0xcc => Control::Ack {
                don: word(12) as u16,
            },
            _ => Control::Other,
        }
    }
}
