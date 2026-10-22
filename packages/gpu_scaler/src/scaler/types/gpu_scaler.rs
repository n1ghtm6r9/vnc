use super::InputTexture;
use crate::YuvTextures;

#[derive(Debug)]
pub struct GpuScaler {
    pub(super) layout: wgpu::BindGroupLayout,
    pub(super) horizontal_pipeline: wgpu::RenderPipeline,
    pub(super) vertical_pipeline: wgpu::RenderPipeline,
    pub(super) nearest_pipeline: wgpu::RenderPipeline,
    pub(super) convert_layout: wgpu::BindGroupLayout,
    pub(super) convert_pipeline: wgpu::RenderPipeline,
    pub(super) convert_params: wgpu::Buffer,
    pub(super) source_params: wgpu::Buffer,
    pub(super) intermediate_params: wgpu::Buffer,
    pub(super) source: Option<InputTexture>,
    pub(super) intermediate: Option<InputTexture>,
    pub(super) yuv: Option<YuvTextures>,
    pub(super) encode_srgb: bool,
    pub(super) max_dimension: u32,
}
