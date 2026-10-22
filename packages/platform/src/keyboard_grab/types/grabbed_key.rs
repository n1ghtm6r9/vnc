#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GrabbedKey {
    pub virtual_key: u32,
    pub down: bool,
}
