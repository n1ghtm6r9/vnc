use std::{
    sync::{atomic::Ordering, mpsc::SyncSender},
    time::Instant,
};

use nvdec::NvdecError;

use super::MediaThread;
use crate::{FrameDecoder, MediaSession};

impl<D: FrameDecoder> MediaThread<D> {
    pub(crate) fn run(mut self, ready: SyncSender<Result<(), NvdecError>>) {
        let decoder = match (self.factories.new_decoder)() {
            Ok(decoder) => decoder,
            Err(error) => {
                self.stop.store(true, Ordering::Relaxed);
                let _ = ready.send(Err(error));
                return;
            }
        };
        let _ = ready.send(Ok(()));
        let session = MediaSession::new(decoder, self.factories.new_secrets, Instant::now());
        self.serve(session);
    }
}
