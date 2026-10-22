#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PlaneShape {
    pub width: u32,
    pub height: u32,
    pub texel_bytes: u32,
    pub format: wgpu::TextureFormat,
}
