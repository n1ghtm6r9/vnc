#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DisplayMode {
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub width: u32,
    pub height: u32,
    pub refresh: f64,
}
