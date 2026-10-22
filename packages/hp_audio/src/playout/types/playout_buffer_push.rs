use super::PlayoutBuffer;
use crate::{FADE_IN_FRAMES, PLAYOUT_LIMIT};

impl PlayoutBuffer {
    pub(crate) fn push(&mut self, sound: &[[f32; 2]]) {
        for frame in sound {
            let gain = if self.fade_in > 0 {
                self.fade_in -= 1;
                1.0 - self.fade_in as f32 / FADE_IN_FRAMES as f32
            } else {
                1.0
            };
            self.frames.push_back(frame.map(|sample| sample * gain));
        }
        if self.level() > PLAYOUT_LIMIT as f64 {
            self.skip_to_target();
        }
    }
}
