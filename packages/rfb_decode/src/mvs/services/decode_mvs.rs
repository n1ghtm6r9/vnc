use std::io::Read;

use rfb::{Rect, read_u32};

use crate::{
    DecodeError, DecodeTarget, MVS_PAYLOAD_BYTES_PER_PIXEL, RectOutcome, ensure_rect_fits,
    read_compressed_payload,
};

pub fn decode_mvs(
    reader: &mut impl Read,
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    let DecodeTarget {
        state, framebuffer, ..
    } = target;
    if !rect.is_empty() {
        ensure_rect_fits(framebuffer, rect)?;
    }
    let length = usize::try_from(read_u32(reader)?).unwrap_or(usize::MAX);
    let limit = rect.area().saturating_mul(MVS_PAYLOAD_BYTES_PER_PIXEL);
    read_compressed_payload(reader, &mut state.compressed, length, limit)?;
    let changed = state.mvs.decode(&state.compressed, rect, framebuffer);
    Ok(RectOutcome::Pixels(changed))
}
