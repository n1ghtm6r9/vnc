use crate::AppleSessionKey;

#[test]
fn test_a_session_key_encrypts_one_aes_128_block() {
    let key = AppleSessionKey(std::array::from_fn(|index| index as u8));
    let plain = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
        0xff,
    ];
    let cipher = [
        0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5,
        0x5a,
    ];
    assert_eq!(
        key.encrypt(&plain),
        cipher,
        "the FIPS-197 example block encrypts to the published ciphertext"
    );
    assert_eq!(
        key.decrypt(&cipher),
        plain,
        "the published ciphertext decrypts back to the example block"
    );
}
