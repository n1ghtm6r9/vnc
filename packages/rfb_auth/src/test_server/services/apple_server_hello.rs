use num_bigint::BigUint;

use crate::{AppleServerOptions, AuthError, dh_public_key};

pub(crate) fn apple_server_hello(options: &AppleServerOptions) -> Result<Vec<u8>, AuthError> {
    let generator = u16::try_from(&BigUint::from_bytes_be(&options.generator)).map_err(|_| {
        AuthError::InvalidServerKey("the generator does not fit in two bytes".to_string())
    })?;
    let key_length = u16::try_from(options.prime.len())
        .ok()
        .filter(|&length| length > 0)
        .ok_or_else(|| {
            AuthError::InvalidServerKey("the prime must be 1 to 65535 bytes long".to_string())
        })?;
    if options.prime.iter().all(|&byte| byte == 0) {
        return Err(AuthError::InvalidServerKey("the prime is zero".to_string()));
    }
    let server_public = dh_public_key(&options.generator, &options.prime, &options.private_key);

    let mut hello = Vec::with_capacity(4 + options.prime.len() + server_public.len());
    hello.extend_from_slice(&generator.to_be_bytes());
    hello.extend_from_slice(&key_length.to_be_bytes());
    hello.extend_from_slice(&options.prime);
    hello.extend_from_slice(&server_public);
    Ok(hello)
}
