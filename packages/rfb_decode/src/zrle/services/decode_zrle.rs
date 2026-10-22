use std::io::Read;

use rfb::{Rect, read_u32};

use crate::{
    DecodeError, DecodeTarget, RectOutcome, ZrleDecoder, ensure_rect_fits, inflate_payload,
    read_compressed_payload, zrle_inflated_limit,
};

pub fn decode_zrle(
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
    let limit = zrle_inflated_limit(rect, converter.compact_bytes_per_pixel());
    read_compressed_payload(reader, &mut state.compressed, length, limit)?;
    let produced = inflate_payload(
        &mut state.zrle_stream,
        &state.compressed,
        &mut state.inflated,
        limit,
    )?;
    ZrleDecoder::new(&state.inflated[..produced], converter, framebuffer).run(rect)?;
    Ok(RectOutcome::Pixels(rect))
}
