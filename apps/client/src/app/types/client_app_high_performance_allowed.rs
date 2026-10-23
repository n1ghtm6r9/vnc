use super::ClientApp;

impl ClientApp {
    pub(super) fn high_performance_allowed(&self) -> bool {
        self.settings.apple_remote_desktop && self.settings.high_performance
    }
}
