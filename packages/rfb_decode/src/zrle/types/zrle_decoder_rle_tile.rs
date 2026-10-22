use rfb::Rect;

use super::ZrleDecoder;
use crate::DecodeError;

impl ZrleDecoder<'_> {
    pub(super) fn rle_tile(&mut self, tile: Rect, palette_size: usize) -> Result<(), DecodeError> {
        self.read_palette(palette_size)?;
        let total = tile.area();
        let mut painted = 0;
        while painted < total {
            let (colour, run) = self.read_run(palette_size)?;
            let left = total - painted;
            if run > left {
                return Err(DecodeError::Corrupt(format!(
                    "a ZRLE run of {run} pixels overruns the {left} pixels left in its tile"
                )));
            }
            self.paint_run(tile, painted..painted + run, colour);
            painted += run;
        }
        Ok(())
    }
}
