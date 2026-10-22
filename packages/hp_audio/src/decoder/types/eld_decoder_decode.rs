use fdk_aac::dec::DecoderError;

use super::EldDecoder;
use crate::PCM_FULL_SCALE;

impl EldDecoder {
    pub(crate) fn decode(&mut self, payload: &[u8]) -> Result<Vec<[f32; 2]>, DecoderError> {
        self.decoder.fill(payload)?;
        self.decoder.decode_frame(&mut self.pcm)?;
        let info = self.decoder.stream_info();
        let channels = usize::try_from(info.numChannels).unwrap_or(0).max(1);
        let frames = usize::try_from(info.frameSize).unwrap_or(0);
        let samples = (frames * channels).min(self.pcm.len());
        Ok(self.pcm[..samples]
            .chunks_exact(channels)
            .map(|frame| {
                let left = f32::from(frame[0]) / PCM_FULL_SCALE;
                let right = frame
                    .get(1)
                    .map_or(left, |&right| f32::from(right) / PCM_FULL_SCALE);
                [left, right]
            })
            .collect())
    }
}
