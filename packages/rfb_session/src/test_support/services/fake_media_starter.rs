use std::sync::mpsc::{self, Receiver};

use crate::StartMedia;
use crate::test_support::{FakeMediaPeer, FakeMediaStart};

pub fn fake_media_starter() -> (StartMedia, Receiver<FakeMediaStart>) {
    let (report, starts) = mpsc::channel();
    let start_media: StartMedia = Box::new(move |options, emit| {
        let (calls_sender, calls) = mpsc::channel();
        let _ = report.send(FakeMediaStart {
            options,
            emit,
            calls,
        });
        Ok(Box::new(FakeMediaPeer {
            calls: calls_sender,
        }))
    });
    (start_media, starts)
}
