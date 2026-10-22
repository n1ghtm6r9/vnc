use std::io::{Cursor, ErrorKind};

use super::recording_stream::RecordingStream;
use crate::{AppleServerOptions, AuthError, serve_apple_auth};

#[test]
fn test_the_test_server_reports_a_client_that_hangs_up() {
    let mut prime = vec![0xff; 16];
    prime[0] = 0x7f;
    for (sent, case) in [(0, "before the ciphertext"), (128, "before the public key")] {
        let mut stream = RecordingStream {
            input: Cursor::new(vec![0x5a; sent]),
            ..Default::default()
        };
        let options = AppleServerOptions {
            generator: vec![2],
            prime: prime.clone(),
            private_key: vec![3],
        };
        let result = serve_apple_auth(&mut stream, options);
        assert!(
            matches!(&result, Err(AuthError::Io(error)) if error.kind() == ErrorKind::UnexpectedEof),
            "a client that hangs up {case} is an unexpected end of the stream, got {result:?}"
        );
    }
}
