use log::warn;
use rfb_session::SessionError;

use super::{ClientApp, LinkState};
use crate::{
    FIRST_CONNECT_ATTEMPTS, LINK_LOST_HEADING, session_error_text, should_reconnect,
    should_reconnect_without_high_performance,
    should_retry_first_connect,
};

impl ClientApp {
    pub(super) fn on_disconnected(&mut self, error: Option<SessionError>) {
        if let LinkState::Connecting { attempt } = self.link
            && should_retry_first_connect(error.as_ref(), attempt)
            && let Some(options) = self.last_options.clone()
        {
            let attempt = attempt + 1;
            warn!("[SESSION] the Mac did not answer in time, connect attempt {} of {FIRST_CONNECT_ATTEMPTS}", attempt + 1);
            self.link = LinkState::Connecting { attempt };
            self.open_session(options);
            return;
        }
        let logged_in = matches!(self.link, LinkState::Online | LinkState::Reconnecting { .. });
        let reason = error.as_ref().map(session_error_text);
        if !should_reconnect(error.as_ref(), logged_in) {
            self.end_session(reason);
            return;
        }
        let online = self.link == LinkState::Online;
        let high_performance = self.last_options.as_ref().is_some_and(|options| options.high_performance.is_some());
        if should_reconnect_without_high_performance(online, high_performance, self.media_picture.is_some()) {
            warn!("[SESSION] the link to the Mac broke while High Performance was starting");
            self.reconnect_without_high_performance();
            return;
        }
        let attempt = match self.link {
            LinkState::Reconnecting { attempt } => attempt.saturating_add(1),
            _ => 0,
        };
        warn!("[SESSION] the link to the Mac is lost");
        self.wait_to_reconnect(attempt, reason, LINK_LOST_HEADING);
    }
}
