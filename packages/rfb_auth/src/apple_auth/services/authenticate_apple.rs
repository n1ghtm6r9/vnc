use std::io::{Read, Write};

use log::debug;

use super::{apple_credentials_block, dh_public_key, dh_shared_secret, read_apple_server_key};
use crate::{
    APPLE_BLOCK_LENGTH, AppleSessionKey, AuthError, Credentials, aes128_ecb_encrypt, fill_random,
    md5_digest,
};

pub fn authenticate_apple(
    stream: &mut (impl Read + Write),
    credentials: &Credentials,
) -> Result<AppleSessionKey, AuthError> {
    let server_key = read_apple_server_key(stream)?;
    let key_length = server_key.prime.len();
    debug!(
        "[AUTH] apple: the server offered a group of {} bits",
        key_length * 8
    );

    let mut private_key = vec![0u8; key_length];
    fill_random(&mut private_key)?;
    let mut filler = [0u8; APPLE_BLOCK_LENGTH];
    fill_random(&mut filler)?;

    let client_public = dh_public_key(&server_key.generator, &server_key.prime, &private_key);
    let shared_secret = dh_shared_secret(&server_key.public, &private_key, &server_key.prime);
    let session_key = md5_digest(&shared_secret);
    let block = apple_credentials_block(&credentials.username, &credentials.password, &filler);
    let ciphertext = aes128_ecb_encrypt(&session_key, &block);

    let mut reply = Vec::with_capacity(APPLE_BLOCK_LENGTH + key_length);
    reply.extend_from_slice(&ciphertext);
    reply.extend_from_slice(&client_public);
    stream.write_all(&reply)?;
    stream.flush()?;
    debug!("[AUTH] apple: the credentials are sent");
    Ok(AppleSessionKey(session_key))
}
