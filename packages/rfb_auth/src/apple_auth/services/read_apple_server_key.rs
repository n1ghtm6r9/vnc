use std::io::Read;

use rfb::{read_bytes, read_u16};

use crate::{APPLE_MAX_KEY_LENGTH, AppleServerKey, AuthError};

pub(crate) fn read_apple_server_key(stream: &mut impl Read) -> Result<AppleServerKey, AuthError> {
    let generator = read_u16(stream)?.to_be_bytes();
    let key_length = usize::from(read_u16(stream)?);
    if !(1..=APPLE_MAX_KEY_LENGTH).contains(&key_length) {
        return Err(AuthError::InvalidServerKey(format!(
            "a key length of {key_length} bytes is outside 1 to {APPLE_MAX_KEY_LENGTH}"
        )));
    }
    let prime = read_bytes(stream, key_length)?;
    if prime.iter().all(|&byte| byte == 0) {
        return Err(AuthError::InvalidServerKey("the prime is zero".to_string()));
    }
    let public = read_bytes(stream, key_length)?;
    Ok(AppleServerKey {
        generator,
        prime,
        public,
    })
}
