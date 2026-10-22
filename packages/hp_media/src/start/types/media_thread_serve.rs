use std::{
    sync::{atomic::Ordering, mpsc::RecvTimeoutError},
    time::Instant,
};

use super::{MediaInput, MediaThread};
use crate::{FrameDecoder, MediaFailure, MediaSession, TICK_INTERVAL};

impl<D: FrameDecoder> MediaThread<D> {
    pub(crate) fn serve(&mut self, mut session: MediaSession<D>) {
        while !session.is_finished() {
            match self.queue.recv_timeout(TICK_INTERVAL) {
                Ok(MediaInput::Message1(payload)) => {
                    session.handle_message1(&payload, Instant::now());
                }
                Ok(MediaInput::Answer(payload)) => session.handle_answer(&payload, Instant::now()),
                Ok(MediaInput::Resize(display)) => session.resize(display, Instant::now()),
                Ok(MediaInput::PlaySound(play)) => self.play_sound(play),
                Ok(MediaInput::Datagram(channel, mut bytes)) => {
                    session.handle_datagram(channel, &mut bytes, Instant::now());
                }
                Ok(MediaInput::LinkFailed(reason)) => session.fail(MediaFailure::Socket(reason)),
                Ok(MediaInput::Stop) | Err(RecvTimeoutError::Disconnected) => break,
                Err(RecvTimeoutError::Timeout) => {}
            }
            session.tick(Instant::now());
            self.flush(&mut session);
        }
        self.stop.store(true, Ordering::Relaxed);
    }
}
