use std::panic::{AssertUnwindSafe, catch_unwind};

use rfb::Rect;

use super::{jpeg_decoder_panic, run_jpeg_decoder};
use crate::DecodeError;

pub(crate) fn decode_jpeg(
    jpeg: &[u8],
    rect: Rect,
    output: &mut Vec<u8>,
) -> Result<usize, DecodeError> {
    catch_unwind(AssertUnwindSafe(|| run_jpeg_decoder(jpeg, rect, output)))
        .unwrap_or_else(|payload| Err(jpeg_decoder_panic(payload)))
}
