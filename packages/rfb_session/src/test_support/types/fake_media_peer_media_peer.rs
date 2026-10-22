use super::{FakeMediaPeer, MediaCall};
use crate::MediaPeer;

impl MediaPeer for FakeMediaPeer {
    fn message1(&self, payload: Vec<u8>) {
        let _ = self.calls.send(MediaCall::Message1(payload));
    }

    fn answer(&self, payload: Vec<u8>) {
        let _ = self.calls.send(MediaCall::Answer(payload));
    }

    fn resize(&self, display: Vec<u8>) {
        let _ = self.calls.send(MediaCall::Resize(display));
    }

    fn play_sound(&self, play: bool) {
        let _ = self.calls.send(MediaCall::PlaySound(play));
    }
}
