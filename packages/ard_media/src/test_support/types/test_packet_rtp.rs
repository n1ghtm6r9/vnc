use super::TestPacket;
use crate::RtpPacket;

impl TestPacket {
    pub fn rtp(&self) -> RtpPacket<'_> {
        RtpPacket {
            ssrc: self.ssrc,
            seq: self.seq,
            timestamp: 0,
            marker: false,
            payload_type: 100,
            extension: &self.extension,
            payload: &self.payload,
        }
    }
}
