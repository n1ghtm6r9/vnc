use rfb::{ENCODING_APPLE_MVS, Rect};

use super::DecodeHarness;
use crate::RectOutcome;
use crate::test_support::length_prefixed;

impl DecodeHarness {
    pub fn decode_mvs(&mut self, rect: Rect, update: &[u8]) -> RectOutcome {
        self.decode(ENCODING_APPLE_MVS, rect, &length_prefixed(update))
            .expect("an MVS update never fails the rect it arrives in")
    }
}
