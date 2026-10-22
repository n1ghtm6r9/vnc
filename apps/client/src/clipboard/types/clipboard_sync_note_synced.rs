use super::ClipboardSync;

impl ClipboardSync {
    pub fn note_synced(&mut self, text: &str) {
        self.last_synced = Some(text.to_owned());
    }
}
