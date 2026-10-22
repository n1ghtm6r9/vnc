#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaledCursor {
    pub width: u16,
    pub height: u16,
    pub hotspot_x: u16,
    pub hotspot_y: u16,
    pub rgba: Vec<u8>,
}
