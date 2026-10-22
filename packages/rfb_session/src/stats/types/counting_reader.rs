pub(crate) struct CountingReader<R> {
    pub(super) inner: R,
    pub(super) count: u64,
}
