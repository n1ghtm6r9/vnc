use std::{mem, time::Instant};

use ard_media::AssemblerEvent;
use log::warn;

use super::MediaSession;
use crate::{FrameDecoder, MediaFailure};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn settle(&mut self, now: Instant) {
        let mut assembled = mem::take(&mut self.assembled);
        for event in assembled.drain(..) {
            match event {
                AssemblerEvent::Picture(picture) => self.show(picture, now),
                AssemblerEvent::Stalled {
                    expected_don,
                    buffered,
                } => {
                    match expected_don {
                        Some(don) => warn!(
                            "[MEDIA] picture {don} never arrived whole, {buffered} pictures after it were waiting"
                        ),
                        None => warn!("[MEDIA] {buffered} pictures piled up before a key picture"),
                    }
                    self.glitch(MediaFailure::Stalled, now);
                }
            }
        }
        self.assembled = assembled;
    }
}
