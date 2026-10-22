#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MediaStats {
    pub bytes_per_second: u64,
    pub frames_per_second: f32,
}
