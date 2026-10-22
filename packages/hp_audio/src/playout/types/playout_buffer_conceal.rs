use std::iter;

use super::PlayoutBuffer;
use crate::{FADE_IN_FRAMES, FADE_OUT_FRAMES, FRAME_SAMPLES, MAX_CONCEALED_FRAMES, PLAYOUT_LIMIT};

impl PlayoutBuffer {
    pub(crate) fn conceal(&mut self, lost: usize) {
        let unplayed = self.frames.len().saturating_sub(self.cursor);
        let fade = FADE_OUT_FRAMES.min(unplayed);
        let start = self.frames.len() - fade;
        for (index, frame) in self.frames.range_mut(start..).enumerate() {
            let gain = 1.0 - (index + 1) as f32 / fade as f32;
            *frame = frame.map(|sample| sample * gain);
        }
        let silence = lost.min(MAX_CONCEALED_FRAMES) * FRAME_SAMPLES;
        self.frames.extend(iter::repeat_n([0.0; 2], silence));
        self.fade_in = FADE_IN_FRAMES;
        self.stats.concealed_frames += lost as u64;
        if self.level() > PLAYOUT_LIMIT as f64 {
            self.skip_to_target();
        }
    }
}
