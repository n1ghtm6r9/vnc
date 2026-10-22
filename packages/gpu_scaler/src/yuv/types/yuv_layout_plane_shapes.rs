use super::{PlaneShape, YuvLayout};

impl YuvLayout {
    pub(crate) fn plane_shapes(self, width: u32, height: u32) -> Vec<PlaneShape> {
        let luma = PlaneShape {
            width,
            height,
            texel_bytes: 1,
            format: wgpu::TextureFormat::R8Unorm,
        };
        match self {
            YuvLayout::Planar444 => vec![luma, luma, luma],
            YuvLayout::Nv12 => vec![
                luma,
                PlaneShape {
                    width: width.div_ceil(2),
                    height: height.div_ceil(2),
                    texel_bytes: 2,
                    format: wgpu::TextureFormat::Rg8Unorm,
                },
            ],
        }
    }
}
