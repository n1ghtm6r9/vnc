use aes::{
    Aes256,
    cipher::{Array, BlockCipherEncrypt, consts::U16},
};

const BATCH: usize = 8;

pub(crate) fn aes_ctr_xor(cipher: &Aes256, counter: [u8; 16], data: &mut [u8]) {
    let base = u16::from_be_bytes([counter[14], counter[15]]);
    let mut keystream = [Array::<u8, U16>::default(); BATCH];
    for (batch, chunk) in data.chunks_mut(16 * BATCH).enumerate() {
        let blocks = chunk.len().div_ceil(16);
        for (offset, block) in keystream[..blocks].iter_mut().enumerate() {
            let number = (batch * BATCH + offset) as u16;
            block[..14].copy_from_slice(&counter[..14]);
            block[14..].copy_from_slice(&base.wrapping_add(number).to_be_bytes());
        }
        cipher.encrypt_blocks(&mut keystream[..blocks]);
        for (byte, key) in chunk.iter_mut().zip(keystream.iter().flatten()) {
            *byte ^= key;
        }
    }
}
