use rfb::{APPLE_PSEUDO_ENCODINGS, ENCODING_APPLE_CURSOR_ALPHA, ENCODING_RAW};

use crate::apple_encodings;

#[test]
fn test_the_apple_pseudo_encodings_are_added_once() {
    let encodings = apple_encodings(&[ENCODING_RAW, ENCODING_APPLE_CURSOR_ALPHA]);

    let mut expected = vec![ENCODING_RAW, ENCODING_APPLE_CURSOR_ALPHA];
    expected.extend(
        APPLE_PSEUDO_ENCODINGS
            .into_iter()
            .filter(|&encoding| encoding != ENCODING_APPLE_CURSOR_ALPHA),
    );
    assert_eq!(
        encodings, expected,
        "the encodings the client asked for keep their order and every Apple pseudo-encoding follows once"
    );
}
