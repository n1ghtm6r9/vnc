#[derive(Debug, Clone)]
pub(crate) struct ColourChannel {
    pub(super) shift: u32,
    pub(super) mask: u32,
    pub(super) table: Vec<u8>,
}
