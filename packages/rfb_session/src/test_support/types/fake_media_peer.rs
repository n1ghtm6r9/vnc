use std::sync::mpsc::Sender;

use super::MediaCall;

pub struct FakeMediaPeer {
    pub calls: Sender<MediaCall>,
}
