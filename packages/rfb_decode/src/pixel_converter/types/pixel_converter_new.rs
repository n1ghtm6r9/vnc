use rfb::PixelFormat;

use super::{ColourChannel, PixelConverter};
use crate::compact_layout_for;

impl PixelConverter {
    pub fn new(format: &PixelFormat) -> PixelConverter {
        let full_channels =
            format.red_max == 255 && format.green_max == 255 && format.blue_max == 255;
        let thirty_two_bit_colour = format.true_colour && format.bits_per_pixel == 32;
        let standard_shifts =
            format.red_shift == 0 && format.green_shift == 8 && format.blue_shift == 16;
        PixelConverter {
            bytes_per_pixel: format.bytes_per_pixel().clamp(1, 4),
            big_endian: format.big_endian,
            true_colour: format.true_colour,
            rgba32_layout: thirty_two_bit_colour
                && !format.big_endian
                && full_channels
                && standard_shifts,
            tight_rgb24: thirty_two_bit_colour && format.depth == 24 && full_channels,
            compact: compact_layout_for(format),
            red: ColourChannel::new(format.red_max, format.red_shift),
            green: ColourChannel::new(format.green_max, format.green_shift),
            blue: ColourChannel::new(format.blue_max, format.blue_shift),
            colour_map: Vec::new(),
        }
    }
}
