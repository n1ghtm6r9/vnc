use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::Instant;

use log::{info, warn};

use crate::{
    DEVICE_CHECK_INTERVAL, PlayoutBuffer, SAMPLE_RATE, SoundPacket, SoundPipeline, Speaker,
};

pub(crate) fn run_audio(queue: Receiver<SoundPacket>) {
    let mut pipeline = match SoundPipeline::new() {
        Ok(pipeline) => pipeline,
        Err(error) => {
            warn!("[AUDIO] the sound decoder cannot start: {error}");
            return;
        }
    };
    let buffer = Arc::new(Mutex::new(PlayoutBuffer::new(SAMPLE_RATE)));
    let mut speaker = Speaker::default();
    loop {
        speaker.refresh(&buffer, Instant::now());
        match queue.recv_timeout(DEVICE_CHECK_INTERVAL) {
            Ok(packet) => pipeline.push(packet, &buffer),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => break,
        }
    }
    drop(speaker);
    let stats = buffer.lock().unwrap_or_else(PoisonError::into_inner).stats;
    info!(
        "[AUDIO] the sound stops: {} underruns, {} frames dropped, {} lost frames covered, {} frames not decoded",
        stats.underruns, stats.dropped_frames, stats.concealed_frames, pipeline.failures
    );
}
