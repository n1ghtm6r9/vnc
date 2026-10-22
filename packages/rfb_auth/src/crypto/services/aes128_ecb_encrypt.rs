use aes::{
    Aes128,
    cipher::{Array, BlockCipherEncrypt, KeyInit, consts::U16},
};

pub fn aes128_ecb_encrypt(key: &[u8; 16], block: &[u8; 128]) -> [u8; 128] {
    let cipher = Aes128::new(key.into());
    let mut encrypted = *block;
    let (chunks, _) = Array::<u8, U16>::slice_as_chunks_mut(&mut encrypted);
    cipher.encrypt_blocks(chunks);
    encrypted
}
