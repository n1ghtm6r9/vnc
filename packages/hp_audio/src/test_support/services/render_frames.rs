use crate::{FRAME_SAMPLES, PlayoutBuffer};

pub fn render_frames(buffer: &mut PlayoutBuffer, count: usize) -> Vec<[f32; 2]> {
    let mut played = Vec::with_capacity(count);
    while played.len() < count {
        let mut output = vec![0.0; 2 * FRAME_SAMPLES.min(count - played.len())];
        buffer.render(&mut output, 2);
        played.extend(output.chunks(2).map(|frame| [frame[0], frame[1]]));
    }
    played
}
