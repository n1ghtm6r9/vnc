use std::io::Read;

use rfb::{Rect, read_u32};

use crate::{
    DecodeError, DecodeTarget, RRE_BATCH_SUBRECTS, RectOutcome, ensure_rect_fits, read_scratch,
};

pub fn decode_rre(
    reader: &mut impl Read,
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    let DecodeTarget {
        converter,
        state,
        framebuffer,
    } = target;
    ensure_rect_fits(framebuffer, rect)?;
    let mut remaining = usize::try_from(read_u32(reader)?).unwrap_or(usize::MAX);
    let pixel_size = converter.bytes_per_pixel();
    let background = converter.rgba(read_scratch(reader, &mut state.wire, pixel_size)?);
    framebuffer.fill_rect(rect, background);
    let entry_size = pixel_size + 8;
    while remaining > 0 {
        let batch = remaining.min(RRE_BATCH_SUBRECTS);
        let data = read_scratch(reader, &mut state.wire, batch * entry_size)?;
        for entry in data.chunks_exact(entry_size) {
            let (colour, geometry) = entry.split_at(pixel_size);
            let (pairs, _) = geometry.as_chunks::<2>();
            let [x, y, width, height] = pairs else {
                return Err(DecodeError::Corrupt(
                    "an RRE subrectangle is missing its geometry".to_owned(),
                ));
            };
            let [x, y, width, height] = [x, y, width, height].map(|pair| u16::from_be_bytes(*pair));
            let inside = u32::from(x) + u32::from(width) <= u32::from(rect.width)
                && u32::from(y) + u32::from(height) <= u32::from(rect.height);
            if !inside {
                return Err(DecodeError::Corrupt(format!(
                    "an RRE subrectangle {width}x{height} at ({x}, {y}) leaves its rectangle"
                )));
            }
            framebuffer.fill_rect(
                Rect::new(rect.x + x, rect.y + y, width, height),
                converter.rgba(colour),
            );
        }
        remaining -= batch;
    }
    Ok(RectOutcome::Pixels(rect))
}
