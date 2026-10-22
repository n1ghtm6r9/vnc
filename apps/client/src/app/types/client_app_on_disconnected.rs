use log::warn;
use rfb_session::SessionError;

use super::{ClientApp, LinkState};
use crate::{session_error_text, should_reconnect};

impl ClientApp {
    pub(super) fn on_disconnected(&mut self, error: Option<SessionError>) {
        let logged_in = matches!(self.link, LinkState::Online | LinkState::Reconnecting { .. });
        let reason = error.as_ref().map(session_error_text);
        if !should_reconnect(error.as_ref(), logged_in) {
            self.end_session(reason);
            return;
        }
        let attempt = match self.link {
            LinkState::Reconnecting { attempt } => attempt.saturating_add(1),
            _ => 0,
        };
        warn!("[SESSION] the link to the Mac is lost");
        self.wait_to_reconnect(attempt, reason);
    }
}
