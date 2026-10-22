use des::{
    Des,
    cipher::{Array, BlockCipherDecrypt, KeyInit, consts::U8},
};

use crate::vnc_auth_response;

#[test]
fn test_the_vnc_response_decrypts_back_to_the_challenge() {
    let challenge: [u8; 16] = std::array::from_fn(|index| (index as u8).wrapping_mul(17));
    let response = vnc_auth_response(&challenge, "secret");
    assert_ne!(
        response, challenge,
        "the response is not the challenge echoed back"
    );

    let bit_reversed_secret = [0xce, 0xa6, 0xc6, 0x4e, 0xa6, 0x2e, 0x00, 0x00];
    let cipher = Des::new(&Array::from(bit_reversed_secret));
    let mut decrypted = response;
    let (blocks, _) = Array::<u8, U8>::slice_as_chunks_mut(&mut decrypted);
    cipher.decrypt_blocks(blocks);
    assert_eq!(
        decrypted, challenge,
        "DES with the bit-reversed, zero-padded password as the key turns both halves of the response back into the challenge"
    );
}
