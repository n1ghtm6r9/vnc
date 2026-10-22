use super::Viewport;

impl Viewport {
    pub const fn new(x: i32, y: i32, width: u32, height: u32) -> Viewport {
        Viewport {
            x,
            y,
            width,
            height,
        }
    }
}
