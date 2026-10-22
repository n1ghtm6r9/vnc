use std::{
    sync::{Arc, atomic::AtomicBool, mpsc},
    thread,
};

use super::spawn_receiver;
use crate::{
    FrameDecoder, MEDIA_CHANNELS, MediaEvent, MediaFactories, MediaHandle, MediaInput, MediaLink,
    MediaOptions, MediaStartError, MediaThread,
};

pub(crate) fn launch_media<D: FrameDecoder + 'static>(
    options: MediaOptions,
    on_event: impl FnMut(MediaEvent) + Send + 'static,
    factories: MediaFactories<D>,
) -> Result<MediaHandle, MediaStartError> {
    let link = Arc::new(MediaLink::bind(&options).map_err(MediaStartError::Bind)?);
    let local_ports = link.local_ports().map_err(MediaStartError::Bind)?;
    let (inputs, queue) = mpsc::channel();
    if options.play_sound {
        let _ = inputs.send(MediaInput::PlaySound(true));
    }
    let stop = Arc::new(AtomicBool::new(false));
    let (ready, started) = mpsc::sync_channel(1);
    let media = MediaThread {
        link: Arc::clone(&link),
        queue,
        on_event: Box::new(on_event),
        stop: Arc::clone(&stop),
        factories,
        sound: None,
    };
    thread::Builder::new()
        .name("hp-media".into())
        .spawn(move || media.run(ready))
        .map_err(MediaStartError::Thread)?;
    match started.recv() {
        Ok(Ok(())) => {}
        Ok(Err(error)) => return Err(MediaStartError::Decoder(error)),
        Err(_) => return Err(MediaStartError::Stopped),
    }
    let handle = MediaHandle {
        inputs,
        stop,
        local_ports,
    };
    for channel in MEDIA_CHANNELS {
        spawn_receiver(
            channel,
            Arc::clone(&link),
            handle.inputs.clone(),
            Arc::clone(&handle.stop),
        )
        .map_err(MediaStartError::Thread)?;
    }
    Ok(handle)
}
