use aes::Aes256;
use hmac::{Hmac, KeyInit};
use sha1::Sha1;

use super::{SrtpCipher, SrtpKeys};

impl SrtpCipher {
    pub(crate) fn new(keys: &SrtpKeys) -> Self {
        Self {
            aes: Aes256::new((&keys.cipher_key).into()),
            mac: <Hmac<Sha1> as KeyInit>::new_from_slice(&keys.auth_key)
                .expect("HMAC takes a key of any length"),
            salt: keys.salt,
        }
    }
}
