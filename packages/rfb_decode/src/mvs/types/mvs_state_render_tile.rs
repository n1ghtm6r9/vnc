use super::MvsState;
use crate::{MVS_BLUE, MVS_LUMA, MVS_RED, chroma_offsets, luma_pixel, transform_plane};

impl MvsState {
    pub(super) fn render_tile(&self) -> [u32; 64] {
        let mut luma = [0u8; 64];
        if let Some(flat) = transform_plane(
            &self.coefficients[MVS_LUMA],
            &self.luma_quantization,
            &mut luma,
        ) {
            luma = [flat; 64];
        }
        let mut blue = [0u8; 64];
        let blue_flat = transform_plane(
            &self.coefficients[MVS_BLUE],
            &self.chroma_quantization,
            &mut blue,
        );
        let mut red = [0u8; 64];
        let red_flat = transform_plane(
            &self.coefficients[MVS_RED],
            &self.chroma_quantization,
            &mut red,
        );
        if let (Some(blue), Some(red)) = (blue_flat, red_flat) {
            let offsets = chroma_offsets(blue, red);
            return luma.map(|value| luma_pixel(value, offsets));
        }
        if let Some(flat) = blue_flat {
            blue = [flat; 64];
        }
        if let Some(flat) = red_flat {
            red = [flat; 64];
        }
        std::array::from_fn(|index| {
            luma_pixel(luma[index], chroma_offsets(blue[index], red[index]))
        })
    }
}
