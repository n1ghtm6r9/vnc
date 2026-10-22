use std::io::Read;

use rfb::Rect;

use crate::{DecodeError, DecodeTarget, RectOutcome, TightDecoder, ensure_rect_fits};

pub fn decode_tight(
    reader: &mut impl Read,
    rect: Rect,
    target: DecodeTarget<'_>,
) -> Result<RectOutcome, DecodeError> {
    ensure_rect_fits(target.framebuffer, rect)?;
    TightDecoder::new(reader, rect, target).run()?;
    Ok(RectOutcome::Pixels(rect))
}
