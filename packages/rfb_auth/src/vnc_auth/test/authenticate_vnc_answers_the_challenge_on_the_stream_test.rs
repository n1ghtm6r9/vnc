use std::io::Cursor;

use crate::{authenticate_vnc, vnc_auth_response};

#[test]
fn test_authenticate_vnc_answers_the_challenge_on_the_stream() {
    let challenge: [u8; 16] =
        std::array::from_fn(|index| (index as u8).wrapping_mul(7).wrapping_add(2));
    let mut stream = Cursor::new(challenge.to_vec());
    authenticate_vnc(&mut stream, "secret").unwrap();
    let bytes = stream.into_inner();
    assert_eq!(
        bytes.len(),
        32,
        "the 16-byte challenge is followed by exactly 16 bytes of response"
    );
    assert_eq!(
        bytes[16..],
        vnc_auth_response(&challenge, "secret"),
        "the bytes written are the DES response to the challenge that was read"
    );
}
