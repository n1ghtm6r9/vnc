use super::{MediaHandle, MediaInput};

impl MediaHandle {
    pub fn message1(&self, payload: Vec<u8>) {
        let _ = self.inputs.send(MediaInput::Message1(payload));
    }
}
