use aes::Aes256;
use hmac::Hmac;
use sha1::Sha1;

pub(crate) struct SrtpCipher {
    pub(crate) aes: Aes256,
    pub(crate) mac: Hmac<Sha1>,
    pub(crate) salt: [u8; 14],
}
