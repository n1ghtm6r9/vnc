use fdk_aac::dec::DecoderError;

use super::SoundPipeline;
use crate::{AudioReorder, EldDecoder};

impl SoundPipeline {
    pub(crate) fn new() -> Result<SoundPipeline, DecoderError> {
        Ok(SoundPipeline {
            reorder: AudioReorder::default(),
            decoder: EldDecoder::new()?,
            failures: 0,
        })
    }
}
