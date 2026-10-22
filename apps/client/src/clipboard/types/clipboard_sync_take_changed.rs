use super::ClipboardSync;

impl ClipboardSync {
    pub fn take_changed(&mut self, local: String) -> Option<String> {
        if local.is_empty() || self.last_synced.as_deref() == Some(local.as_str()) {
            return None;
        }
        self.last_synced = Some(local.clone());
        Some(local)
    }
}
