use rfb::Rect;
use zune_jpeg::JpegDecoder;
use zune_jpeg::zune_core::bytestream::ZCursor;
use zune_jpeg::zune_core::colorspace::ColorSpace;
use zune_jpeg::zune_core::options::DecoderOptions;

use super::corrupt_jpeg;
use crate::DecodeError;

pub(crate) fn run_jpeg_decoder(
    jpeg: &[u8],
    rect: Rect,
    output: &mut Vec<u8>,
) -> Result<usize, DecodeError> {
    let options = DecoderOptions::default()
        .jpeg_set_out_colorspace(ColorSpace::RGB)
        .set_max_width(usize::from(rect.width))
        .set_max_height(usize::from(rect.height));
    let mut decoder = JpegDecoder::new_with_options(ZCursor::new(jpeg), options);
    decoder.decode_headers().map_err(corrupt_jpeg)?;
    let wanted = (usize::from(rect.width), usize::from(rect.height));
    match decoder.dimensions() {
        Some(dimensions) if dimensions == wanted => {}
        other => {
            return Err(DecodeError::Corrupt(format!(
                "a Tight JPEG of {other:?} does not fit its {}x{} rect",
                wanted.0, wanted.1
            )));
        }
    }
    let size = decoder
        .output_buffer_size()
        .ok_or_else(|| DecodeError::Corrupt("the Tight JPEG size overflows".to_owned()))?;
    if output.len() < size {
        output.resize(size, 0);
    }
    decoder
        .decode_into(&mut output[..size])
        .map_err(corrupt_jpeg)?;
    Ok(size)
}
