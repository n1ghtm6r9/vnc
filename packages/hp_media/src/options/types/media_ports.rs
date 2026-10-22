#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaPorts {
    pub local: [u16; 2],
    pub remote: [u16; 2],
}
