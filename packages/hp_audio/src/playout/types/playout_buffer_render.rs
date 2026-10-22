use super::PlayoutBuffer;
use crate::{
    FULL_CORRECTION_OFFSET, KERNEL_HALF, LEVEL_SMOOTHING, MAX_RATE_CORRECTION, PLAYOUT_TARGET,
    SAMPLE_RATE, write_device_frame,
};

impl PlayoutBuffer {
    pub(crate) fn render(&mut self, output: &mut [f32], channels: usize) {
        let channels = channels.max(1);
        let level = self.level();
        let target = PLAYOUT_TARGET as f64;
        if !self.playing && level >= target {
            self.playing = true;
            self.smoothed_level = level;
        }
        let count = output.len() / channels;
        self.smoothed_level +=
            (level - self.smoothed_level) * (count as f64 / LEVEL_SMOOTHING).min(1.0);
        let offset = (self.smoothed_level - target) / FULL_CORRECTION_OFFSET as f64;
        let correction =
            (offset * MAX_RATE_CORRECTION).clamp(-MAX_RATE_CORRECTION, MAX_RATE_CORRECTION);
        let step = f64::from(SAMPLE_RATE) / f64::from(self.output_rate) * (1.0 + correction);
        let frames = self.frames.make_contiguous();
        for device_frame in output.chunks_mut(channels) {
            let sound = if self.playing && self.cursor + KERNEL_HALF < frames.len() {
                let sound = self.kernel.interpolate(frames, self.cursor, self.phase);
                self.phase += step;
                let whole = self.phase.floor();
                self.cursor += whole as usize;
                self.phase -= whole;
                sound
            } else {
                if self.playing {
                    self.playing = false;
                    self.stats.underruns += 1;
                }
                [0.0; 2]
            };
            write_device_frame(device_frame, sound);
        }
        self.trim_history();
    }
}
