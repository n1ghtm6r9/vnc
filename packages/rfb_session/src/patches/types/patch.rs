use rfb::Rect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Patch {
    pub rect: Rect,
    pub pixels: Vec<u32>,
}
