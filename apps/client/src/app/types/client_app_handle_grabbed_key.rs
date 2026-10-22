use platform::GrabbedKey;

use super::{ClientApp, LinkState};

impl ClientApp {
    pub(super) fn handle_grabbed_key(&mut self, key: GrabbedKey) {
        if self.link != LinkState::Online {
            return;
        }
        if let Some(command) = self.router.grabbed_key(key) {
            self.session.send(command);
        }
    }
}
