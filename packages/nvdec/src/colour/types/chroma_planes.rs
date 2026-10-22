#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ChromaPlanes {
    pub(crate) count: usize,
    pub(crate) row_bytes: usize,
    pub(crate) rows: usize,
}
