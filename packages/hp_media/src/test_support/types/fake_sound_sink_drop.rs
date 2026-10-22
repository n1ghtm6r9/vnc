use super::{FakeSoundSink, SoundNotice};

impl Drop for FakeSoundSink {
    fn drop(&mut self) {
        let _ = self.notices.send(SoundNotice::Closed);
    }
}
