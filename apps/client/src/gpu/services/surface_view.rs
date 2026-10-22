pub fn surface_view(surface_texture: &wgpu::SurfaceTexture, format: wgpu::TextureFormat) -> wgpu::TextureView {
    surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {
        format: Some(format),
        ..Default::default()
    })
}
