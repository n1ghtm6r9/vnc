#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SessionStats {
    pub bytes_per_second: u64,
    pub updates_per_second: f32,
    pub last_encoding: i32,
}
