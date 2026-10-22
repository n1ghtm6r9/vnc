#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct ConvertParams {
    pub origin: [i32; 2],
    pub chroma_shift: u32,
    pub interleaved_chroma: u32,
    pub rows: [[f32; 4]; 3],
    pub primaries: [[f32; 4]; 3],
}
