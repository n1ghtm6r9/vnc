use fdk_aac::dec::{Decoder, DecoderError, Transport};

use super::EldDecoder;
use crate::{AUDIO_SPECIFIC_CONFIG, MAX_PCM_SAMPLES};

impl EldDecoder {
    pub(crate) fn new() -> Result<EldDecoder, DecoderError> {
        let mut decoder = Decoder::new(Transport::Raw);
        decoder.config_raw(&AUDIO_SPECIFIC_CONFIG)?;
        Ok(EldDecoder {
            decoder,
            pcm: vec![0; MAX_PCM_SAMPLES],
        })
    }
}
