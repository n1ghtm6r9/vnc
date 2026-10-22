use crate::{RTP_HEADER_LENGTH, RtpLayout};

pub fn rtp_layout(packet: &[u8]) -> Option<RtpLayout> {
    let first = *packet.first()?;
    let header_length = RTP_HEADER_LENGTH + 4 * usize::from(first & 0x0f);
    if packet.len() < header_length {
        return None;
    }
    let mut payload_offset = header_length;
    if first & 0x10 != 0 {
        let words = packet.get(header_length + 2..header_length + 4)?;
        payload_offset += 4 + 4 * usize::from(u16::from_be_bytes([words[0], words[1]]));
    }
    (payload_offset <= packet.len()).then_some(RtpLayout {
        header_length,
        payload_offset,
    })
}
