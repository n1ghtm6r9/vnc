use super::SrtpCipher;
use crate::aes_ctr_xor;

impl SrtpCipher {
    pub(crate) fn crypt(&self, ssrc: u32, index: u64, data: &mut [u8]) {
        let mut counter = [0u8; 16];
        counter[4..8].copy_from_slice(&ssrc.to_be_bytes());
        counter[8..14].copy_from_slice(&index.to_be_bytes()[2..]);
        for (byte, salt) in counter.iter_mut().zip(self.salt) {
            *byte ^= salt;
        }
        aes_ctr_xor(&self.aes, counter, data);
    }
}
