use super::render_frames;
use crate::{FRAME_SAMPLES, PlayoutBuffer};

pub fn play_steadily(buffer: &mut PlayoutBuffer, frames: usize) {
    for _ in 0..frames {
        buffer.push(&[[0.25, 0.25]; FRAME_SAMPLES]);
        render_frames(buffer, FRAME_SAMPLES);
    }
}
