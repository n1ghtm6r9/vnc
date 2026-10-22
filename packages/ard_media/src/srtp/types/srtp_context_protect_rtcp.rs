use super::SrtpContext;
use crate::{RTCP_HEADER_LENGTH, SRTCP_E_BIT, SRTCP_INDEX_MAX, SRTP_TAG_LENGTH};

impl SrtpContext {
    pub fn protect_rtcp(&mut self, plain: &[u8]) -> Option<Vec<u8>> {
        if plain.len() < RTCP_HEADER_LENGTH || self.rtcp_index >= SRTCP_INDEX_MAX {
            return None;
        }
        self.rtcp_index += 1;
        let ssrc = u32::from_be_bytes([plain[4], plain[5], plain[6], plain[7]]);
        let mut out = Vec::with_capacity(plain.len() + 4 + SRTP_TAG_LENGTH);
        out.extend_from_slice(plain);
        self.rtcp.crypt(
            ssrc,
            u64::from(self.rtcp_index),
            &mut out[RTCP_HEADER_LENGTH..],
        );
        out.extend_from_slice(&(SRTCP_E_BIT | self.rtcp_index).to_be_bytes());
        let tag = self.rtcp.tag(&[&out]);
        out.extend_from_slice(&tag);
        Some(out)
    }
}
