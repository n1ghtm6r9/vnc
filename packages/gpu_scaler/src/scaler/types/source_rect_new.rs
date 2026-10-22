use super::SourceRect;

impl SourceRect {
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> SourceRect {
        SourceRect {
            x,
            y,
            width,
            height,
        }
    }
}
