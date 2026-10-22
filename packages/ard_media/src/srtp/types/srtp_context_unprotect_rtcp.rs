use super::SrtpContext;
use crate::{RTCP_HEADER_LENGTH, SRTCP_E_BIT, SRTCP_INDEX_MAX, SRTP_TAG_LENGTH};

impl SrtpContext {
    pub fn unprotect_rtcp<'a>(&self, packet: &'a mut [u8]) -> Option<&'a [u8]> {
        let end = packet
            .len()
            .checked_sub(SRTP_TAG_LENGTH + 4)
            .filter(|end| *end >= RTCP_HEADER_LENGTH)?;
        let (authenticated, tag) = packet.split_at(end + 4);
        if self.rtcp.tag(&[authenticated]) != tag {
            return None;
        }
        let word = u32::from_be_bytes([
            packet[end],
            packet[end + 1],
            packet[end + 2],
            packet[end + 3],
        ]);
        let ssrc = u32::from_be_bytes([packet[4], packet[5], packet[6], packet[7]]);
        let plain = &mut packet[..end];
        if word & SRTCP_E_BIT != 0 {
            let index = u64::from(word & SRTCP_INDEX_MAX);
            self.rtcp
                .crypt(ssrc, index, &mut plain[RTCP_HEADER_LENGTH..]);
        }
        Some(plain)
    }
}
