use crate::{aes128_ecb_decrypt, aes128_ecb_encrypt};

#[test]
fn test_aes_ecb_round_trips() {
    let key: [u8; 16] = std::array::from_fn(|index| (index as u8).wrapping_mul(11).wrapping_add(7));
    let block: [u8; 128] =
        std::array::from_fn(|index| (index as u8).wrapping_mul(5).wrapping_add(3));
    let ciphertext = aes128_ecb_encrypt(&key, &block);
    assert_ne!(ciphertext, block, "encryption changes the bytes");
    assert_eq!(
        aes128_ecb_decrypt(&key, &ciphertext),
        block,
        "decrypting with the same key returns the original 128 bytes"
    );
}
