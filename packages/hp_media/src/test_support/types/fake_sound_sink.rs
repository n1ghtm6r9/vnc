use std::sync::mpsc::Sender;

use super::SoundNotice;

pub struct FakeSoundSink {
    pub notices: Sender<SoundNotice>,
}
