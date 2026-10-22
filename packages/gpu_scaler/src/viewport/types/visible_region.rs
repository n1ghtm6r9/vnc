#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VisibleRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
