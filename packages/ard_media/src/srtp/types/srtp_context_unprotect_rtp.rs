use super::{RtpPacket, SrtpContext};
use crate::{SRTP_TAG_LENGTH, estimate_roc, rtp_layout};

impl SrtpContext {
    pub fn unprotect_rtp<'a>(&mut self, packet: &'a mut [u8]) -> Option<RtpPacket<'a>> {
        let end = packet.len().checked_sub(SRTP_TAG_LENGTH)?;
        let layout = rtp_layout(&packet[..end])?;
        let seq = u16::from_be_bytes([packet[2], packet[3]]);
        let timestamp = u32::from_be_bytes([packet[4], packet[5], packet[6], packet[7]]);
        let ssrc = u32::from_be_bytes([packet[8], packet[9], packet[10], packet[11]]);
        let highest = self.received.get(&ssrc).copied();
        let roc = estimate_roc(highest, seq);
        let (authenticated, tag) = packet.split_at(end);
        if self.rtp.tag(&[authenticated, &roc.to_be_bytes()]) != tag {
            return None;
        }
        let index = (u64::from(roc) << 16) | u64::from(seq);
        if highest.is_none_or(|highest| highest < index) {
            self.received.insert(ssrc, index);
        }
        let marker = packet[1] & 0x80 != 0;
        let payload_type = packet[1] & 0x7f;
        let (header, payload) = packet[..end].split_at_mut(layout.payload_offset);
        self.rtp.crypt(ssrc, index, payload);
        let header: &'a [u8] = header;
        let payload: &'a [u8] = payload;
        Some(RtpPacket {
            ssrc,
            seq,
            timestamp,
            marker,
            payload_type,
            extension: &header[layout.header_length..],
            payload,
        })
    }
}
