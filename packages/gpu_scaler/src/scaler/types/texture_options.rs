pub(crate) struct TextureOptions {
    pub label: &'static str,
    pub width: u32,
    pub height: u32,
    pub format: wgpu::TextureFormat,
    pub usage: wgpu::TextureUsages,
}
