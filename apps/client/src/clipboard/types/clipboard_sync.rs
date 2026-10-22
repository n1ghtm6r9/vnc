#[derive(Debug, Default)]
pub struct ClipboardSync {
    pub(super) last_synced: Option<String>,
}
