use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BufferSize, SampleFormat, StreamConfig};
use log::{debug, info};

use super::AudioOutput;
use crate::{PlayoutBuffer, SAMPLE_RATE, build_output};

impl AudioOutput {
    pub(crate) fn open(buffer: &Arc<Mutex<PlayoutBuffer>>) -> Result<AudioOutput, String> {
        let device = cpal::default_host()
            .default_output_device()
            .ok_or_else(|| "there is no sound output device".to_owned())?;
        let name = device.description().map_or_else(
            |_| "the default sound device".to_owned(),
            |description| description.name().to_owned(),
        );
        let failed = Arc::new(AtomicBool::new(false));
        let wanted = StreamConfig {
            channels: 2,
            sample_rate: SAMPLE_RATE,
            buffer_size: BufferSize::Default,
        };
        let (stream, config) = match build_output::<f32>(&device, wanted, buffer, &failed) {
            Ok(stream) => (stream, wanted),
            Err(error) => {
                debug!("[AUDIO] {name} does not take 48 kHz stereo: {error}");
                let supported = device
                    .default_output_config()
                    .map_err(|error| format!("{name} has no output format: {error}"))?;
                let config = supported.config();
                let stream = match supported.sample_format() {
                    SampleFormat::F32 => build_output::<f32>(&device, config, buffer, &failed),
                    SampleFormat::I16 => build_output::<i16>(&device, config, buffer, &failed),
                    SampleFormat::I32 => build_output::<i32>(&device, config, buffer, &failed),
                    SampleFormat::U16 => build_output::<u16>(&device, config, buffer, &failed),
                    format => return Err(format!("{name} wants {format:?} samples")),
                }
                .map_err(|error| format!("{name} cannot play: {error}"))?;
                (stream, config)
            }
        };
        stream
            .play()
            .map_err(|error| format!("{name} cannot start: {error}"))?;
        info!(
            "[AUDIO] the Mac's sound plays on {name}: {} Hz, {} channels",
            config.sample_rate, config.channels
        );
        Ok(AudioOutput {
            _stream: stream,
            device: device.id().ok(),
            failed,
        })
    }
}
