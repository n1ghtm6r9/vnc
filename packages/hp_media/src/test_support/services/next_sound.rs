use std::sync::mpsc::Receiver;

use crate::test_support::{EVENT_TIMEOUT, SoundNotice};

pub fn next_sound(notices: &Receiver<SoundNotice>) -> SoundNotice {
    notices
        .recv_timeout(EVENT_TIMEOUT)
        .expect("the player hears from the media in time")
}
