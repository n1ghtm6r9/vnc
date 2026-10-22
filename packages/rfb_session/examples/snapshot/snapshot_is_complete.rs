use crate::Snapshot;

impl Snapshot {
    pub(crate) fn is_complete(&self) -> bool {
        self.first_frame.is_some() && self.stats.is_some()
    }
}
