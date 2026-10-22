use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use cpal::{DeviceId, Stream};

pub(crate) struct AudioOutput {
    pub(crate) _stream: Stream,
    pub(crate) device: Option<DeviceId>,
    pub(crate) failed: Arc<AtomicBool>,
}
