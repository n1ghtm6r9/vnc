use std::sync::mpsc::Receiver;

use crate::test_support::{EVENT_TIMEOUT, FakeMediaStart};

pub fn next_media_start(starts: &Receiver<FakeMediaStart>) -> FakeMediaStart {
    starts
        .recv_timeout(EVENT_TIMEOUT)
        .expect("the session starts the media")
}
