use super::YuvLayout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct YuvShape {
    pub layout: YuvLayout,
    pub width: u32,
    pub height: u32,
}
