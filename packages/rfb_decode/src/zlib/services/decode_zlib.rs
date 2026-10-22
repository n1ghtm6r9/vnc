use std::io::Read;

use rfb::{Rect, read_u32};

use crate::{
    DecodeError, DecodeTarget, RectOutcome, ensure_rect_fits, inflate_payload,
    read_compressed_payload, write_raw_rows,
};

pub fn decode_zlib(
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
    let length = usize::try_from(read_u32(reader)?).unwrap_or(usize::MAX);
    let expected = rect.area() * converter.bytes_per_pixel();
    read_compressed_payload(reader, &mut state.compressed, length, expected)?;
    let produced = inflate_payload(
        &mut state.zlib_stream,
        &state.compressed,
        &mut state.inflated,
        expected,
    )?;
    if produced != expected {
        return Err(DecodeError::Corrupt(format!(
            "the zlib data inflates to {produced} bytes where the rect needs {expected}"
        )));
    }
    write_raw_rows(converter, framebuffer, rect, &state.inflated[..expected]);
    Ok(RectOutcome::Pixels(rect))
}
