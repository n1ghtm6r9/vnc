use super::DisplayMode;

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualDisplay {
    pub name: String,
    pub largest_pixel_width: u32,
    pub largest_pixel_height: u32,
    pub modes: Vec<DisplayMode>,
}
