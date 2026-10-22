use std::sync::mpsc::Receiver;

use hp_media::MediaOptions;

use super::MediaCall;
use crate::MediaCallback;

pub struct FakeMediaStart {
    pub options: MediaOptions,
    pub emit: MediaCallback,
    pub calls: Receiver<MediaCall>,
}
