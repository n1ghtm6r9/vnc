use std::sync::{Mutex, PoisonError};

use super::SoundPipeline;
use crate::{PlayoutBuffer, SoundPacket};

impl SoundPipeline {
    pub(crate) fn push(&mut self, packet: SoundPacket, buffer: &Mutex<PlayoutBuffer>) {
        let frames = self.reorder.push(packet);
        if frames.is_empty() {
            return;
        }
        let decoded: Vec<_> = frames
            .iter()
            .map(|frame| (frame.lost, self.decode(&frame.payload)))
            .collect();
        let mut buffer = buffer.lock().unwrap_or_else(PoisonError::into_inner);
        for (lost, sound) in decoded {
            if lost > 0 {
                buffer.conceal(lost);
            }
            match sound {
                Some(sound) => buffer.push(&sound),
                None => buffer.conceal(1),
            }
        }
    }
}
