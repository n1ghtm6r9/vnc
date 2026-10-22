use log::{debug, warn};
use rfb::ServerMessage;

use super::SessionReader;
use crate::{Outgoing, SessionError, SessionEvent, latin1_text};

impl SessionReader {
    pub(super) fn read_message(
        &mut self,
        on_event: &mut dyn FnMut(SessionEvent),
    ) -> Result<(), SessionError> {
        match ServerMessage::read(&mut self.input)? {
            ServerMessage::FramebufferUpdate { rect_count } => {
                self.read_update(rect_count, on_event)?;
            }
            ServerMessage::SetColourMapEntries {
                first_colour,
                colours,
            } => self.converter.set_colour_map(first_colour, &colours),
            ServerMessage::Bell => on_event(SessionEvent::Bell),
            ServerMessage::ServerCutText { text } => {
                on_event(SessionEvent::ClipboardText(latin1_text(&text)));
            }
            ServerMessage::OversizedCutText { length } => {
                warn!(
                    "[SESSION] the Mac clipboard text of {length} bytes is too long, it is skipped"
                );
            }
            ServerMessage::AppleStateChange { status } => {
                self.apply_state_change(status, on_event)?;
            }
            ServerMessage::AppleClipboard { session_id, text } => {
                self.queue(Outgoing::ClipboardSession(session_id));
                if let Some(text) = text {
                    on_event(SessionEvent::ClipboardText(text));
                }
            }
            ServerMessage::AppleDisplayInfo { size, geometry } => {
                debug!("[SESSION] the displays of the Mac changed, now {size:?}");
                if let Some(geometry) = geometry {
                    on_event(SessionEvent::DisplayGeometry(geometry));
                }
                self.refetch_screen();
            }
            ServerMessage::AppleAck | ServerMessage::AppleNop | ServerMessage::AppleDrag => {}
        }
        Ok(())
    }
}
