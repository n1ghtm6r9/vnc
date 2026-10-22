#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OrderedFrame {
    pub(crate) lost: usize,
    pub(crate) payload: Vec<u8>,
}
