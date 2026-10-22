use super::SrtpContext;
use crate::{SRTP_TAG_LENGTH, estimate_roc, rtp_layout};

impl SrtpContext {
    pub fn protect_rtp(&mut self, plain: &[u8]) -> Option<Vec<u8>> {
        let layout = rtp_layout(plain)?;
        let seq = u16::from_be_bytes([plain[2], plain[3]]);
        let ssrc = u32::from_be_bytes([plain[8], plain[9], plain[10], plain[11]]);
        let highest = self.sent.get(&ssrc).copied();
        let roc = estimate_roc(highest, seq);
        let index = (u64::from(roc) << 16) | u64::from(seq);
        if highest.is_none_or(|highest| highest < index) {
            self.sent.insert(ssrc, index);
        }
        let mut out = Vec::with_capacity(plain.len() + SRTP_TAG_LENGTH);
        out.extend_from_slice(plain);
        self.rtp
            .crypt(ssrc, index, &mut out[layout.payload_offset..]);
        let tag = self.rtp.tag(&[&out, &roc.to_be_bytes()]);
        out.extend_from_slice(&tag);
        Some(out)
    }
}
