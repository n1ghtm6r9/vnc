use std::io::{Cursor, ErrorKind};

use super::recording_stream::RecordingStream;
use crate::{
    AppleServerOptions, AuthError, aes128_ecb_encrypt, dh_public_key, dh_shared_secret, md5_digest,
    serve_apple_auth,
};

#[test]
fn test_the_test_server_refuses_credentials_that_are_not_utf8() {
    let mut prime = vec![0xff; 16];
    prime[0] = 0x7f;
    let server_public = dh_public_key(&[2], &prime, &[3]);
    let client_private = [5];
    let shared_secret = dh_shared_secret(&server_public, &client_private, &prime);
    let mut reply = aes128_ecb_encrypt(&md5_digest(&shared_secret), &[0xff; 128]).to_vec();
    reply.extend_from_slice(&dh_public_key(&[2], &prime, &client_private));
    let mut stream = RecordingStream {
        input: Cursor::new(reply),
        ..Default::default()
    };
    let options = AppleServerOptions {
        generator: vec![2],
        prime,
        private_key: vec![3],
    };
    let result = serve_apple_auth(&mut stream, options);
    assert!(
        matches!(&result, Err(AuthError::Io(error)) if error.kind() == ErrorKind::InvalidData),
        "a user name that is not UTF-8 is invalid data, got {result:?}"
    );
}
