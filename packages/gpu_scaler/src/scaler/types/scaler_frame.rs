use crate::{ScaleFilter, Viewport};

pub struct ScalerFrame<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub target: &'a wgpu::TextureView,
    pub target_width: u32,
    pub target_height: u32,
    pub viewport: Viewport,
    pub filter: ScaleFilter,
    pub background: wgpu::Color,
}
