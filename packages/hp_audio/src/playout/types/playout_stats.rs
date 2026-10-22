#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct PlayoutStats {
    pub(crate) underruns: u64,
    pub(crate) dropped_frames: u64,
    pub(crate) concealed_frames: u64,
}
