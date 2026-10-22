use super::MediaFailure;

impl MediaFailure {
    pub fn needs_reconnect(&self) -> bool {
        !matches!(self, MediaFailure::NoMessage1)
    }
}
