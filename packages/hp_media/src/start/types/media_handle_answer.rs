use super::{MediaHandle, MediaInput};

impl MediaHandle {
    pub fn answer(&self, payload: Vec<u8>) {
        let _ = self.inputs.send(MediaInput::Answer(payload));
    }
}
