use log::info;

use super::MediaThread;
use crate::FrameDecoder;

impl<D: FrameDecoder> MediaThread<D> {
    pub(crate) fn play_sound(&mut self, play: bool) {
        if play == self.sound.is_some() {
            return;
        }
        if play {
            info!("[MEDIA] the Mac's sound now plays here");
            self.sound = Some((self.factories.new_sound)());
        } else {
            info!("[MEDIA] the Mac's sound is no longer played here");
            self.sound = None;
        }
    }
}
