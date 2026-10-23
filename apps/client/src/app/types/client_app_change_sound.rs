use rfb_session::ClientCommand;

use super::ClientApp;
use crate::{
    BACK_TO_HIGH_PERFORMANCE_STATUS, STANDARD_MODE_FOR_SOUND_STATUS, SoundChange, SoundPlace, high_performance_options,
    sound_change,
};

impl ClientApp {
    pub(super) fn change_sound(&mut self, place: SoundPlace) {
        let Some(options) = &self.last_options else {
            return;
        };
        match sound_change(place, options.high_performance.is_some(), self.high_performance_allowed()) {
            SoundChange::Play(play) => {
                if let Some(wanted) = self.last_options.as_mut().and_then(|options| options.high_performance.as_mut()) {
                    wanted.play_sound = play;
                }
                self.session.send(ClientCommand::PlaySound(play));
            }
            SoundChange::LeaveHighPerformance => self.reconnect_for_sound(None, STANDARD_MODE_FOR_SOUND_STATUS),
            SoundChange::EnterHighPerformance => {
                let wanted = high_performance_options(self.high_performance_display(), self.settings.retina, place);
                self.reconnect_for_sound(Some(wanted), BACK_TO_HIGH_PERFORMANCE_STATUS);
            }
            SoundChange::Keep => {}
        }
    }
}
