use des::{
    Des,
    cipher::{Array, BlockCipherEncrypt, KeyInit, consts::U8},
};

use super::vnc_des_key;

pub fn vnc_auth_response(challenge: &[u8; 16], password: &str) -> [u8; 16] {
    let cipher = Des::new(&Array::from(vnc_des_key(password)));
    let mut response = *challenge;
    let (halves, _) = Array::<u8, U8>::slice_as_chunks_mut(&mut response);
    cipher.encrypt_blocks(halves);
    response
}
