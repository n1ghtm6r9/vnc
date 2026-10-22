use std::io::Read;

use rfb::Rect;

use super::HextileDecoder;
use crate::{DecodeError, HEXTILE_TILE_SIZE};

impl<R: Read> HextileDecoder<'_, R> {
    pub(crate) fn run(&mut self, rect: Rect) -> Result<(), DecodeError> {
        let step = usize::from(HEXTILE_TILE_SIZE);
        for y in (0..rect.height).step_by(step) {
            for x in (0..rect.width).step_by(step) {
                let width = (rect.width - x).min(HEXTILE_TILE_SIZE);
                let height = (rect.height - y).min(HEXTILE_TILE_SIZE);
                self.decode_tile(Rect::new(rect.x + x, rect.y + y, width, height))?;
            }
        }
        Ok(())
    }
}
