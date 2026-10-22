use aes::{
    Aes128,
    cipher::{Array, BlockCipherEncrypt, KeyInit, consts::U16},
};

use super::AppleSessionKey;

impl AppleSessionKey {
    pub fn encrypt(&self, block: &[u8; 16]) -> [u8; 16] {
        let key = &self.0;
        let cipher = Aes128::new(key.into());
        let mut encrypted = *block;
        let (chunks, _) = Array::<u8, U16>::slice_as_chunks_mut(&mut encrypted);
        cipher.encrypt_blocks(chunks);
        encrypted
    }
}
