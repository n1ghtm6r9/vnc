use std::io::Cursor;

use rfb::{Rect, RectHeader};

use super::DecodeHarness;
use crate::{DecodeError, DecodeTarget, RectOutcome, decode_rect};

impl DecodeHarness {
    pub fn decode_counting(
        &mut self,
        encoding: i32,
        rect: Rect,
        payload: &[u8],
    ) -> (Result<RectOutcome, DecodeError>, usize) {
        let mut reader = Cursor::new(payload);
        let header = RectHeader { rect, encoding };
        let target = DecodeTarget {
            converter: &self.converter,
            state: &mut self.state,
            framebuffer: &mut self.framebuffer,
        };
        let outcome = decode_rect(&mut reader, &header, target);
        let consumed = usize::try_from(reader.position()).unwrap_or(usize::MAX);
        (outcome, consumed)
    }
}
