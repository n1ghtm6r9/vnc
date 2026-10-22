use std::time::Instant;

use log::info;

use super::{ClientApp, LinkState};
use crate::fit_display_to_window;

impl ClientApp {
    pub(super) fn reconnect_when_due(&mut self) {
        let LinkState::Waiting { attempt, at } = self.link else {
            return;
        };
        if at > Instant::now() {
            return;
        }
        let Some(mut options) = self.last_options.clone() else {
            self.end_session(None);
            return;
        };
        if let Some(window) = &self.window {
            fit_display_to_window(&mut options, window.inner_size());
        }
        info!("[SESSION] reconnect attempt {} to {}:{}", attempt + 1, options.host, options.port);
        self.link = LinkState::Reconnecting { attempt };
        self.open_session(options);
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
