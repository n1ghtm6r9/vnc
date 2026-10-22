use rfb::PixelFormat;

use super::colour_bits;
use crate::CompactLayout;

pub(crate) fn compact_layout_for(format: &PixelFormat) -> CompactLayout {
    let eligible = format.bits_per_pixel == 32 && format.depth <= 24 && format.true_colour;
    let Some(bits) = colour_bits(format).filter(|_| eligible) else {
        return CompactLayout::Full;
    };
    if bits >> 24 == 0 {
        CompactLayout::LeastSignificantThree
    } else if bits & 0xFF == 0 {
        CompactLayout::MostSignificantThree
    } else {
        CompactLayout::Full
    }
}
