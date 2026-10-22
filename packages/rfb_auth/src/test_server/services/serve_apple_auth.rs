use std::io::{Read, Write};

use log::debug;
use rfb::read_bytes;

use super::{apple_server_hello, read_apple_credentials};
use crate::{
    APPLE_BLOCK_LENGTH, AppleLogin, AppleServerOptions, AppleSessionKey, AuthError,
    aes128_ecb_decrypt, dh_shared_secret, md5_digest,
};

pub fn serve_apple_auth(
    stream: &mut (impl Read + Write),
    options: AppleServerOptions,
) -> Result<AppleLogin, AuthError> {
    let hello = apple_server_hello(&options)?;
    stream.write_all(&hello)?;
    stream.flush()?;

    let mut ciphertext = [0u8; APPLE_BLOCK_LENGTH];
    stream.read_exact(&mut ciphertext)?;
    let client_public = read_bytes(stream, options.prime.len())?;

    let shared_secret = dh_shared_secret(&client_public, &options.private_key, &options.prime);
    let session_key = md5_digest(&shared_secret);
    let block = aes128_ecb_decrypt(&session_key, &ciphertext);
    let credentials = read_apple_credentials(&block)?;
    debug!("[AUTH] apple server: the credentials are received");
    Ok(AppleLogin {
        credentials,
        key: AppleSessionKey(session_key),
    })
}
