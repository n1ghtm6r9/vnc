use rfb::{ENCODING_TIGHT, Rect};

use super::seeded_rgb_bytes;
use crate::DecodeError;
use crate::test_support::{DecodeHarness, ZlibFixture};

pub fn fresh_tight_stream_after_history(control: u8) -> Result<(), DecodeError> {
    let mut harness = DecodeHarness::new(4, 3);
    let mut history = ZlibFixture::new();
    for seed in [1, 2] {
        let mut payload = vec![0x00u8];
        payload.extend_from_slice(&history.tight_payload(&seeded_rgb_bytes(seed, 36)));
        harness
            .decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 3), &payload)
            .expect("rects on one continuing stream decode");
    }
    let mut payload = vec![control];
    payload.extend_from_slice(&ZlibFixture::new().tight_payload(&seeded_rgb_bytes(3, 36)));
    harness
        .decode(ENCODING_TIGHT, Rect::new(0, 0, 4, 3), &payload)
        .map(|_| ())
}
