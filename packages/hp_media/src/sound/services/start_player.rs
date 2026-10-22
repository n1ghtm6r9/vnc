use hp_audio::AudioPlayer;

use crate::SoundSink;

pub(crate) fn start_player() -> Box<dyn SoundSink> {
    Box::new(AudioPlayer::start())
}
