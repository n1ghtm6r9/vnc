use rfb::Rect;

use super::DecodeHarness;
use crate::{DecodeError, RectOutcome};

impl DecodeHarness {
    pub fn decode(
        &mut self,
        encoding: i32,
        rect: Rect,
        payload: &[u8],
    ) -> Result<RectOutcome, DecodeError> {
        let (outcome, consumed) = self.decode_counting(encoding, rect, payload);
        if outcome.is_ok() {
            assert_eq!(
                consumed,
                payload.len(),
                "the decoder must read the whole payload of a good rect"
            );
        }
        outcome
    }
}
