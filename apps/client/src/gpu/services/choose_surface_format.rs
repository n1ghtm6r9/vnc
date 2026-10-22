use wgpu::TextureFormat;

pub fn choose_surface_format(supported: &[TextureFormat]) -> Option<TextureFormat> {
    [TextureFormat::Bgra8Unorm, TextureFormat::Rgba8Unorm]
        .into_iter()
        .find(|format| supported.contains(format))
        .or_else(|| supported.iter().copied().find(|format| !format.is_srgb()))
        .or_else(|| supported.first().copied())
}
