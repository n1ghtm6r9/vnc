use std::io::Read;

use rfb::Rect;

use crate::{
    DecodeError, DecodeTarget, RAW_BAND_BYTES, RectOutcome, ensure_rect_fits, read_scratch,
    write_raw_rows,
};

pub fn decode_raw(
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
    let row_bytes = usize::from(rect.width) * converter.bytes_per_pixel();
    if let Some(rows_in_band) = RAW_BAND_BYTES.checked_div(row_bytes) {
        let rows_per_band = u16::try_from(rows_in_band.max(1)).unwrap_or(u16::MAX);
        let mut first_row = 0;
        while first_row < rect.height {
            let rows = rows_per_band.min(rect.height - first_row);
            let band = Rect::new(rect.x, rect.y + first_row, rect.width, rows);
            let data = read_scratch(reader, &mut state.wire, usize::from(rows) * row_bytes)?;
            write_raw_rows(converter, framebuffer, band, data);
            first_row += rows;
        }
    }
    Ok(RectOutcome::Pixels(rect))
}
