#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct PassParams {
    pub origin: [i32; 2],
    pub input_size: [i32; 2],
    pub output_size: [i32; 2],
    pub encode_srgb: u32,
    pub padding: u32,
}
