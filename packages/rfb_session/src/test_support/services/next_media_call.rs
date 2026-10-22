use crate::test_support::{EVENT_TIMEOUT, FakeMediaStart, MediaCall};

pub fn next_media_call(media: &FakeMediaStart) -> MediaCall {
    media
        .calls
        .recv_timeout(EVENT_TIMEOUT)
        .expect("the session passes the next media rect on")
}
