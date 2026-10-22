use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};

use cpal::traits::DeviceTrait;
use cpal::{Device, ErrorKind, FromSample, OutputCallbackInfo, SizedSample, Stream, StreamConfig};
use log::{debug, warn};

use crate::PlayoutBuffer;

pub(crate) fn build_output<T: SizedSample + FromSample<f32>>(
    device: &Device,
    config: StreamConfig,
    buffer: &Arc<Mutex<PlayoutBuffer>>,
    failed: &Arc<AtomicBool>,
) -> Result<Stream, cpal::Error> {
    buffer
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .set_output_rate(config.sample_rate);
    let buffer = Arc::clone(buffer);
    let failed = Arc::clone(failed);
    let channels = usize::from(config.channels);
    let mut mix = Vec::new();
    device.build_output_stream(
        config,
        move |output: &mut [T], _: &OutputCallbackInfo| {
            mix.resize(output.len(), 0.0);
            buffer
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .render(&mut mix, channels);
            for (sample, &value) in output.iter_mut().zip(&mix) {
                *sample = T::from_sample(value);
            }
        },
        move |error: cpal::Error| {
            if matches!(error.kind(), ErrorKind::Xrun | ErrorKind::RealtimeDenied) {
                debug!("[AUDIO] the sound device reports: {error}");
            } else {
                failed.store(true, Ordering::Relaxed);
                warn!("[AUDIO] the sound device stopped: {error}");
            }
        },
        None,
    )
}
