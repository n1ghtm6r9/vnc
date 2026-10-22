use super::{linear_to_srgb, resample_rows, srgb_to_linear, transpose};
use crate::test_support::LinearImage;

pub fn reference_resample(
    source: &[u8],
    source_size: (u32, u32),
    output_size: (u32, u32),
) -> Vec<u8> {
    let pixels = source
        .as_chunks::<4>()
        .0
        .iter()
        .map(|texel| {
            [
                srgb_to_linear(texel[0]),
                srgb_to_linear(texel[1]),
                srgb_to_linear(texel[2]),
            ]
        })
        .collect();
    let image = LinearImage {
        width: source_size.0,
        height: source_size.1,
        pixels,
    };
    let horizontal = resample_rows(&image, output_size.0);
    let resampled = transpose(&resample_rows(&transpose(&horizontal), output_size.1));
    resampled
        .pixels
        .iter()
        .flat_map(|pixel| {
            [
                linear_to_srgb(pixel[0]),
                linear_to_srgb(pixel[1]),
                linear_to_srgb(pixel[2]),
                255,
            ]
        })
        .collect()
}
