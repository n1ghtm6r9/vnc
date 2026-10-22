use super::ZrleDecoder;
use crate::{DecodeError, ZRLE_LOW_SEVEN_BITS, ZRLE_RLE_FLAG};

impl ZrleDecoder<'_> {
    pub(super) fn read_run(&mut self, palette_size: usize) -> Result<(u32, usize), DecodeError> {
        if palette_size == 0 {
            let colour = self.read_pixel()?;
            return Ok((colour, self.read_run_length()?));
        }
        let entry = self.take_byte()?;
        let index = usize::from(entry & ZRLE_LOW_SEVEN_BITS);
        let colour = match self.palette.get(index) {
            Some(&colour) if index < palette_size => colour,
            _ => {
                return Err(DecodeError::Corrupt(format!(
                    "the ZRLE palette index {index} is outside the palette of {palette_size} colours"
                )));
            }
        };
        let run = if entry & ZRLE_RLE_FLAG != 0 {
            self.read_run_length()?
        } else {
            1
        };
        Ok((colour, run))
    }
}
