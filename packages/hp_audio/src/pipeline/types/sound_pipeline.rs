use crate::{AudioReorder, EldDecoder};

pub(crate) struct SoundPipeline {
    pub(crate) reorder: AudioReorder,
    pub(crate) decoder: EldDecoder,
    pub(crate) failures: u64,
}
