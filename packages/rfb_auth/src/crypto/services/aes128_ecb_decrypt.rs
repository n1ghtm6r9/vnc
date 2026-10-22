use aes::{
    Aes128,
    cipher::{Array, BlockCipherDecrypt, KeyInit, consts::U16},
};

pub fn aes128_ecb_decrypt(key: &[u8; 16], block: &[u8; 128]) -> [u8; 128] {
    let cipher = Aes128::new(key.into());
    let mut decrypted = *block;
    let (chunks, _) = Array::<u8, U16>::slice_as_chunks_mut(&mut decrypted);
    cipher.decrypt_blocks(chunks);
    decrypted
}
