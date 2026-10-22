use std::sync::mpsc;
use std::thread;

use log::warn;

use super::AudioPlayer;
use crate::run_audio;

impl AudioPlayer {
    pub fn start() -> AudioPlayer {
        let (packets, queue) = mpsc::channel();
        if let Err(error) = thread::Builder::new()
            .name("hp-audio".into())
            .spawn(move || run_audio(queue))
        {
            warn!("[AUDIO] the sound thread cannot start: {error}");
        }
        AudioPlayer { packets }
    }
}
