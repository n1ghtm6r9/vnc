use rfb::PixelFormat;

use super::channel_bits;

pub(crate) fn colour_bits(format: &PixelFormat) -> Option<u32> {
    let red = channel_bits(format.red_max, format.red_shift)?;
    let green = channel_bits(format.green_max, format.green_shift)?;
    let blue = channel_bits(format.blue_max, format.blue_shift)?;
    Some(red | green | blue)
}
