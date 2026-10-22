use super::{MediaHandle, MediaInput};

impl MediaHandle {
    pub fn play_sound(&self, play: bool) {
        let _ = self.inputs.send(MediaInput::PlaySound(play));
    }
}
