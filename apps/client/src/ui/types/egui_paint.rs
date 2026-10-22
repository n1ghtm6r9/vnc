use winit::dpi::PhysicalSize;

pub struct EguiPaint<'a> {
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
    pub encoder: &'a mut wgpu::CommandEncoder,
    pub target: &'a wgpu::TextureView,
    pub size: PhysicalSize<u32>,
}
