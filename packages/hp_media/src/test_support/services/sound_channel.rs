use std::sync::mpsc::{self, Receiver};

use crate::{
    SoundFactory,
    test_support::{FakeSoundSink, SoundNotice},
};

pub fn sound_channel() -> (SoundFactory, Receiver<SoundNotice>) {
    let (sender, notices) = mpsc::channel();
    let new_sound: SoundFactory = Box::new(move || {
        let _ = sender.send(SoundNotice::Opened);
        Box::new(FakeSoundSink {
            notices: sender.clone(),
        })
    });
    (new_sound, notices)
}
