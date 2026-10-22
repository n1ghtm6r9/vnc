use des::{
    Des,
    cipher::{Array, BlockCipherEncrypt, KeyInit, consts::U8},
};

use crate::vnc_auth_response;

#[test]
fn test_a_password_byte_is_bit_reversed_into_the_des_key() {
    let challenge: [u8; 16] =
        std::array::from_fn(|index| (index as u8).wrapping_mul(29).wrapping_add(5));
    let encrypt_with_key = |key: [u8; 8]| {
        let cipher = Des::new(&Array::from(key));
        let mut encrypted = challenge;
        let (blocks, _) = Array::<u8, U8>::slice_as_chunks_mut(&mut encrypted);
        cipher.encrypt_blocks(blocks);
        encrypted
    };
    let response = vnc_auth_response(&challenge, "\u{1}");
    assert_eq!(
        response,
        encrypt_with_key([0x80, 0, 0, 0, 0, 0, 0, 0]),
        "the password byte 0x01 reaches DES as the key byte 0x80"
    );
    assert_ne!(
        response,
        encrypt_with_key([0x01, 0, 0, 0, 0, 0, 0, 0]),
        "the password byte is not used as the key byte unchanged"
    );
}
