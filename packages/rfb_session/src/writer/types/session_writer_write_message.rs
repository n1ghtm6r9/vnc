use std::io::{self, Write};

use rfb::{
    write_apple_auto_update, write_apple_clipboard_request, write_framebuffer_update_request,
};

use super::{Outgoing, SessionWriter};

impl SessionWriter {
    pub(super) fn write_message(&mut self, message: Outgoing) -> io::Result<()> {
        match message {
            Outgoing::Command(command) => self.write_command(command),
            Outgoing::UpdateRequest { incremental, rect } => {
                self.screen = rect;
                if self.media_streaming() {
                    return Ok(());
                }
                write_framebuffer_update_request(&mut self.output, incremental, rect)
            }
            Outgoing::AutoUpdate { rect } => {
                self.screen = rect;
                write_apple_auto_update(&mut self.output, rect)
            }
            Outgoing::ClipboardSession(session_id) => {
                self.clipboard_session = session_id;
                Ok(())
            }
            Outgoing::ClipboardRequest => {
                write_apple_clipboard_request(&mut self.output, self.clipboard_session)
            }
            Outgoing::ClipboardResend => self.resend_clipboard(),
            Outgoing::MediaMessage(message) => self.output.write_all(&message),
            Outgoing::Stop => Ok(()),
        }
    }
}
