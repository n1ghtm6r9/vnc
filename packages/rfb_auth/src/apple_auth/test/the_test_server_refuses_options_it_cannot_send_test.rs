use super::recording_stream::RecordingStream;
use crate::{AppleServerOptions, AuthError, serve_apple_auth};

#[test]
fn test_the_test_server_refuses_options_it_cannot_send() {
    let cases = [
        (vec![1, 0, 0], vec![23], "a generator wider than two bytes"),
        (vec![2], Vec::new(), "an empty prime"),
        (vec![2], vec![0, 0], "a zero prime"),
    ];
    for (generator, prime, case) in cases {
        let mut stream = RecordingStream::default();
        let options = AppleServerOptions {
            generator,
            prime,
            private_key: vec![3],
        };
        let result = serve_apple_auth(&mut stream, options);
        assert!(
            matches!(result, Err(AuthError::InvalidServerKey(_))),
            "{case} is refused, got {result:?}"
        );
        assert!(
            stream.writes.is_empty(),
            "{case} sends nothing to the client"
        );
    }
}
