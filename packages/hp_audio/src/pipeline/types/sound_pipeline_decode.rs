use log::warn;

use super::SoundPipeline;

impl SoundPipeline {
    pub(super) fn decode(&mut self, payload: &[u8]) -> Option<Vec<[f32; 2]>> {
        match self.decoder.decode(payload) {
            Ok(sound) => Some(sound),
            Err(error) => {
                if self.failures == 0 {
                    warn!(
                        "[AUDIO] a sound frame of {} bytes cannot be decoded: {error}",
                        payload.len()
                    );
                }
                self.failures += 1;
                None
            }
        }
    }
}
