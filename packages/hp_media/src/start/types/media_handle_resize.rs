use super::{MediaHandle, MediaInput};

impl MediaHandle {
    pub fn resize(&self, display: Vec<u8>) {
        let _ = self.inputs.send(MediaInput::Resize(display));
    }
}
