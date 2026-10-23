use std::time::Instant;

use log::warn;

use super::{ClientApp, LinkState};
use crate::{Notice, Phrase, reconnect_delay};

impl ClientApp {
    pub(super) fn wait_to_reconnect(&mut self, attempt: usize, reason: Option<Notice>, heading: Phrase) {
        let delay = reconnect_delay(attempt);
        warn!("[SESSION] reconnecting in {} s", delay.as_secs());
        self.session.stop();
        self.router.release_all();
        self.restore_default_pointer();
        self.stats = None;
        self.media_picture = None;
        self.mac_display = None;
        self.link_error = reason;
        self.link_heading = heading;
        self.link = LinkState::Waiting {
            attempt,
            at: Instant::now() + delay,
        };
        self.update_keyboard_grab();
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
