use crate::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RectHeader {
    pub rect: Rect,
    pub encoding: i32,
}
