use super::SessionReader;

impl SessionReader {
    pub(super) fn refetch_screen(&self) {
        self.request_update(false);
        if self.apple_remote_desktop {
            self.renew_pushed_updates();
        }
    }
}
