use hmac::Mac;

use super::SrtpCipher;
use crate::SRTP_TAG_LENGTH;

impl SrtpCipher {
    pub(crate) fn tag(&self, parts: &[&[u8]]) -> [u8; SRTP_TAG_LENGTH] {
        let mut mac = self.mac.clone();
        for part in parts {
            mac.update(part);
        }
        let digest = mac.finalize().into_bytes();
        digest[..SRTP_TAG_LENGTH]
            .try_into()
            .expect("an HMAC-SHA1 digest is longer than the tag")
    }
}
