use std::io::Read;

use rfb::Rect;

use crate::{DecodeError, DecodeTarget, HextileDecoder, RectOutcome, ensure_rect_fits};

pub fn decode_hextile(
    reader: &mut impl Read,
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    ensure_rect_fits(target.framebuffer, rect)?;
    let mut decoder = HextileDecoder::new(reader, target);
    decoder.run(rect)?;
    Ok(RectOutcome::Pixels(rect))
}
