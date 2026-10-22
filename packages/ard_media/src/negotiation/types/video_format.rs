#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoFormat {
    pub ssrc: u32,
    pub width: u32,
    pub height: u32,
    pub stripes: u32,
}
