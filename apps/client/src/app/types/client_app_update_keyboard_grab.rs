use super::{ClientApp, LinkState};

impl ClientApp {
    pub(super) fn update_keyboard_grab(&self) {
        if let Some(grab) = &self.keyboard_grab {
            grab.set_active(self.link == LinkState::Online);
        }
    }
}
