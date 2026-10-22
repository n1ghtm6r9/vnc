use aes::{
    Aes128,
    cipher::{Array, BlockCipherDecrypt, KeyInit, consts::U16},
};

use super::AppleSessionKey;

impl AppleSessionKey {
    pub fn decrypt(&self, block: &[u8; 16]) -> [u8; 16] {
        let key = &self.0;
        let cipher = Aes128::new(key.into());
        let mut decrypted = *block;
        let (chunks, _) = Array::<u8, U16>::slice_as_chunks_mut(&mut decrypted);
        cipher.decrypt_blocks(chunks);
        decrypted
    }
}
