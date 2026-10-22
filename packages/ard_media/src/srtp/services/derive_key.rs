use aes::Aes256;

use super::aes_ctr_xor;

pub(crate) fn derive_key<const N: usize>(cipher: &Aes256, salt: &[u8; 14], label: u8) -> [u8; N] {
    let mut counter = [0u8; 16];
    counter[..14].copy_from_slice(salt);
    counter[7] ^= label;
    let mut key = [0u8; N];
    aes_ctr_xor(cipher, counter, &mut key);
    key
}
