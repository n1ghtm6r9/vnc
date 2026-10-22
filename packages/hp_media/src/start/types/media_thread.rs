use std::sync::{Arc, atomic::AtomicBool, mpsc::Receiver};

use super::{MediaFactories, MediaInput};
use crate::{MediaEvent, MediaLink, SoundSink};

pub(crate) struct MediaThread<D> {
    pub(crate) link: Arc<MediaLink>,
    pub(crate) queue: Receiver<MediaInput>,
    pub(crate) on_event: Box<dyn FnMut(MediaEvent) + Send>,
    pub(crate) stop: Arc<AtomicBool>,
    pub(crate) factories: MediaFactories<D>,
    pub(crate) sound: Option<Box<dyn SoundSink>>,
}
