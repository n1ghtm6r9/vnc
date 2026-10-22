use rfb::{
    ENCODING_APPLE_CURSOR_ALPHA, ENCODING_APPLE_CURSOR_POSITION, ENCODING_APPLE_DEVICE_INFO,
    ENCODING_APPLE_DISPLAY_INFO, ENCODING_APPLE_DISPLAY_INFO_2, ENCODING_APPLE_KEYBOARD_INPUT,
    ENCODING_APPLE_MVS, ENCODING_APPLE_USER_INFO, ENCODING_COPY_RECT, ENCODING_CURSOR,
    ENCODING_DESKTOP_SIZE, ENCODING_EXTENDED_DESKTOP_SIZE, ENCODING_HEXTILE, ENCODING_RAW,
    ENCODING_RRE, ENCODING_TIGHT, ENCODING_ZLIB, ENCODING_ZRLE, Rect,
};

use crate::test_support::{
    DecodeHarness, random_below, random_bytes, random_compressed_payload, random_mvs_payload,
    random_tight_payload,
};

#[test]
fn test_garbage_never_makes_a_decoder_panic() {
    let encodings = [
        ENCODING_RAW,
        ENCODING_COPY_RECT,
        ENCODING_RRE,
        ENCODING_HEXTILE,
        ENCODING_ZLIB,
        ENCODING_ZRLE,
        ENCODING_TIGHT,
        ENCODING_CURSOR,
        ENCODING_DESKTOP_SIZE,
        ENCODING_EXTENDED_DESKTOP_SIZE,
        ENCODING_APPLE_MVS,
        ENCODING_APPLE_CURSOR_POSITION,
        ENCODING_APPLE_DISPLAY_INFO,
        ENCODING_APPLE_USER_INFO,
        ENCODING_APPLE_CURSOR_ALPHA,
        ENCODING_APPLE_DISPLAY_INFO_2,
        ENCODING_APPLE_DEVICE_INFO,
        ENCODING_APPLE_KEYBOARD_INPUT,
    ];
    let mut state = 0x9E37_79B9_7F4A_7C15u64;
    let mut accepted = 0usize;
    for _ in 0..150 {
        for encoding in encodings {
            let coordinate = |state: &mut u64, limit: usize| {
                u16::try_from(random_below(state, limit)).expect("a small number fits a u16")
            };
            let rect = Rect::new(
                coordinate(&mut state, 70),
                coordinate(&mut state, 70),
                coordinate(&mut state, 40),
                coordinate(&mut state, 40),
            );
            let payload = match encoding {
                ENCODING_ZLIB | ENCODING_ZRLE => random_compressed_payload(&mut state),
                ENCODING_TIGHT => random_tight_payload(&mut state, rect),
                ENCODING_APPLE_MVS => random_mvs_payload(&mut state),
                _ => {
                    let length = random_below(&mut state, 500);
                    random_bytes(&mut state, length)
                }
            };
            let mut harness = DecodeHarness::new(64, 64);
            let (outcome, _) = harness.decode_counting(encoding, rect, &payload);
            accepted += usize::from(outcome.is_ok());
        }
    }
    assert!(
        accepted > 100,
        "some of the noise is a valid rect, so the decoders are exercised past their first check, accepted {accepted}"
    );
}
