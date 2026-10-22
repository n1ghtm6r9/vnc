use std::io::Read;

use rfb::{Rect, read_u8};

use super::HextileDecoder;
use crate::{
    DecodeError, HEXTILE_ANY_SUBRECTS, HEXTILE_BACKGROUND_SPECIFIED, HEXTILE_FOREGROUND_SPECIFIED,
    HEXTILE_RAW, HEXTILE_SUBRECTS_COLOURED,
};

impl<R: Read> HextileDecoder<'_, R> {
    pub(super) fn decode_tile(&mut self, tile: Rect) -> Result<(), DecodeError> {
        let flags = read_u8(self.reader)?;
        if flags & HEXTILE_RAW != 0 {
            return self.raw_tile(tile);
        }
        if flags & HEXTILE_BACKGROUND_SPECIFIED != 0 {
            self.background = self.read_pixel()?;
        }
        if flags & HEXTILE_FOREGROUND_SPECIFIED != 0 {
            self.foreground = self.read_pixel()?;
        }
        self.framebuffer.fill_rect(tile, self.background);
        if flags & HEXTILE_ANY_SUBRECTS != 0 {
            let count = read_u8(self.reader)?;
            self.draw_subrects(tile, count, flags & HEXTILE_SUBRECTS_COLOURED != 0)?;
        }
        Ok(())
    }
}
