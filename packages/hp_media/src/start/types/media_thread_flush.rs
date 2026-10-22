use log::debug;

use super::MediaThread;
use crate::{FrameDecoder, MediaSession};

impl<D: FrameDecoder> MediaThread<D> {
    pub(crate) fn flush(&mut self, session: &mut MediaSession<D>) {
        let effects = session.effects.take();
        for datagram in &effects.datagrams {
            if let Err(error) = self.link.send(datagram) {
                debug!(
                    "[MEDIA] feedback to the Mac's {} port was not sent: {error}",
                    datagram.channel.name()
                );
            }
        }
        if let Some(sound) = &mut self.sound {
            for packet in effects.sounds {
                sound.play(packet);
            }
        }
        for event in effects.events {
            (self.on_event)(event);
        }
    }
}
