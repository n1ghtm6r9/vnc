use super::{YuvColour, YuvLayout};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YuvStripe<'a> {
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub plane_width: u32,
    pub plane_height: u32,
    pub layout: YuvLayout,
    pub colour: YuvColour,
    pub planes: &'a [u8],
}
