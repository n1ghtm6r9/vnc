use crate::{ScaleFilter, Viewport};

pub struct TargetOptions {
    pub format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
    pub viewport: Viewport,
    pub filter: ScaleFilter,
    pub background: wgpu::Color,
}
