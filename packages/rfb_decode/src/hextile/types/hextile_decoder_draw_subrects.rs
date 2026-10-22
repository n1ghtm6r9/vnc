use std::io::Read;

use rfb::Rect;

use super::HextileDecoder;
use crate::{DecodeError, read_scratch};

impl<R: Read> HextileDecoder<'_, R> {
    pub(super) fn draw_subrects(
        &mut self,
        tile: Rect,
        count: u8,
        coloured: bool,
    ) -> Result<(), DecodeError> {
        let pixel_size = if coloured {
            self.converter.bytes_per_pixel()
        } else {
            0
        };
        let entry_size = pixel_size + 2;
        let data = read_scratch(
            self.reader,
            &mut self.state.wire,
            usize::from(count) * entry_size,
        )?;
        for entry in data.chunks_exact(entry_size) {
            let (colour, geometry) = entry.split_at(pixel_size);
            let colour = if coloured {
                self.converter.rgba(colour)
            } else {
                self.foreground
            };
            let (position, size) = (geometry[0], geometry[1]);
            let (x, y) = (u16::from(position >> 4), u16::from(position & 0x0F));
            let (width, height) = (u16::from(size >> 4) + 1, u16::from(size & 0x0F) + 1);
            if x + width > tile.width || y + height > tile.height {
                return Err(DecodeError::Corrupt(format!(
                    "a hextile subrectangle {width}x{height} at ({x}, {y}) leaves its tile"
                )));
            }
            self.framebuffer
                .fill_rect(Rect::new(tile.x + x, tile.y + y, width, height), colour);
        }
        Ok(())
    }
}
