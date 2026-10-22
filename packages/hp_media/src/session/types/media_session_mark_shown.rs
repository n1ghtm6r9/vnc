use std::mem;

use log::info;

use super::{MediaPhase, MediaSession};
use crate::{FrameDecoder, MediaEvent, MediaState};

impl<D: FrameDecoder> MediaSession<D> {
    pub(crate) fn mark_shown(&mut self) {
        let Some(stream) = self.phase.stream_mut() else {
            return;
        };
        if stream.shown {
            return;
        }
        stream.shown = true;
        let video = stream.answer.video;
        self.phase = mem::replace(&mut self.phase, MediaPhase::Finished).into_streaming();
        info!(
            "[MEDIA] High Performance shows {}x{} in {} stripes",
            video.width, video.height, video.stripes
        );
        self.effects
            .events
            .push(MediaEvent::State(MediaState::Streaming {
                width: video.width,
                height: video.height,
                stripes: video.stripes,
            }));
    }
}
