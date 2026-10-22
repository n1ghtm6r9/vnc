use rfb::{ENCODING_TIGHT, Rect};

use crate::DecodeError;
use crate::test_support::{DecodeHarness, compact_length};

#[test]
fn test_a_tight_jpeg_that_makes_the_jpeg_decoder_panic_is_refused() {
    let jpegs: [&[u8]; 2] = [
        include_bytes!("../../../fixtures/jpeg_that_panics_in_the_idct.jpg"),
        include_bytes!("../../../fixtures/jpeg_that_panics_in_the_upsampler.jpg"),
    ];
    for jpeg in jpegs {
        let mut payload = vec![0x90u8];
        payload.extend_from_slice(&compact_length(jpeg.len()));
        payload.extend_from_slice(jpeg);
        let mut harness = DecodeHarness::new(16, 16);
        let result = harness.decode(ENCODING_TIGHT, Rect::new(0, 0, 16, 16), &payload);
        assert!(
            matches!(result, Err(DecodeError::Corrupt(_))),
            "a JPEG the decoder panics on ends the rect with an error, got {result:?}"
        );
    }
}
