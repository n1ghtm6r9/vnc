use crate::OrderedFrame;

pub fn frame_seqs(frames: &[OrderedFrame]) -> Vec<(usize, u16)> {
    frames
        .iter()
        .map(|frame| {
            (
                frame.lost,
                u16::from_be_bytes([frame.payload[0], frame.payload[1]]),
            )
        })
        .collect()
}
