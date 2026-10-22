use log::{debug, info};
use rfb::{
    APPLE_STATE_CURSOR_HIDDEN, APPLE_STATE_CURSOR_VISIBLE, APPLE_STATE_LOCAL_USER_CLOSED,
    APPLE_STATE_PASTEBOARD_CHANGED, APPLE_STATE_PASTEBOARD_DATA_NEEDED, APPLE_STATE_SLEEP,
    APPLE_STATE_TICKLE, APPLE_STATE_WAKE,
};

use super::SessionReader;
use crate::{Outgoing, SessionError, SessionEvent};

impl SessionReader {
    pub(super) fn apply_state_change(
        &mut self,
        status: u16,
        on_event: &mut dyn FnMut(SessionEvent),
    ) -> Result<(), SessionError> {
        match status {
            APPLE_STATE_LOCAL_USER_CLOSED => return Err(SessionError::ClosedOnTheMac),
            APPLE_STATE_PASTEBOARD_CHANGED => self.queue(Outgoing::ClipboardRequest),
            APPLE_STATE_PASTEBOARD_DATA_NEEDED => self.queue(Outgoing::ClipboardResend),
            APPLE_STATE_TICKLE => self.renew_pushed_updates(),
            APPLE_STATE_SLEEP => info!("[SESSION] the Mac went to sleep"),
            APPLE_STATE_WAKE => info!("[SESSION] the Mac woke up"),
            APPLE_STATE_CURSOR_HIDDEN | APPLE_STATE_CURSOR_VISIBLE => {
                let visible = status == APPLE_STATE_CURSOR_VISIBLE;
                if self.cursor_visible.replace(visible) != Some(visible) {
                    on_event(SessionEvent::CursorVisibilityChanged { visible });
                }
            }
            other => debug!("[SESSION] the Mac reports state {other}"),
        }
        Ok(())
    }
}
