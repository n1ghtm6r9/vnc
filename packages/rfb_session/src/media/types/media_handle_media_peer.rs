use hp_media::MediaHandle;

use super::MediaPeer;

impl MediaPeer for MediaHandle {
    fn message1(&self, payload: Vec<u8>) {
        MediaHandle::message1(self, payload);
    }

    fn answer(&self, payload: Vec<u8>) {
        MediaHandle::answer(self, payload);
    }

    fn resize(&self, display: Vec<u8>) {
        MediaHandle::resize(self, display);
    }

    fn play_sound(&self, play: bool) {
        MediaHandle::play_sound(self, play);
    }
}
