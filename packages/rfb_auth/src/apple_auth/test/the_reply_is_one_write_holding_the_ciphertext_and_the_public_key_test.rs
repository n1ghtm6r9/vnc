use std::io::Cursor;

use super::recording_stream::RecordingStream;
use crate::{Credentials, aes128_ecb_decrypt, authenticate_apple, dh_shared_secret, md5_digest};

#[test]
fn test_the_reply_is_one_write_holding_the_ciphertext_and_the_public_key() {
    let prime = [
        0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff,
    ];
    let server_public = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8];
    let mut hello = vec![0, 2, 0, 16];
    hello.extend_from_slice(&prime);
    hello.extend_from_slice(&server_public);
    let mut stream = RecordingStream {
        input: Cursor::new(hello),
        ..Default::default()
    };

    authenticate_apple(&mut stream, &Credentials::new("bogdan", "пароль")).unwrap();

    assert_eq!(
        stream.writes.len(),
        1,
        "the ciphertext and the key leave in a single write"
    );
    assert_eq!(stream.flushes, 1, "the write is flushed once");
    let reply = &stream.writes[0];
    assert_eq!(
        reply.len(),
        128 + 16,
        "128 bytes of ciphertext, then a key as long as the prime"
    );
    let (ciphertext, client_public) = reply.split_at(128);
    let shared = dh_shared_secret(client_public, &[3], &prime);
    let block = aes128_ecb_decrypt(&md5_digest(&shared), ciphertext.try_into().unwrap());
    assert_eq!(
        &block[..7],
        b"bogdan\0",
        "the user name decrypts with the secret derived from the server key"
    );
    assert_eq!(
        &block[64..77],
        "пароль\0".as_bytes(),
        "the password decrypts as UTF-8 at offset 64"
    );
}
