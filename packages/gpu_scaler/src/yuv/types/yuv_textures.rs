use super::YuvShape;

#[derive(Debug)]
pub(crate) struct YuvTextures {
    pub shape: YuvShape,
    pub planes: Vec<wgpu::Texture>,
    pub bind_group: wgpu::BindGroup,
}
