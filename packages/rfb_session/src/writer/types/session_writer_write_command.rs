use std::io;

use rfb::{AppleScroll, write_apple_set_input_source, write_pointer_event};

use super::SessionWriter;
use crate::{ClientCommand, apple_pointer_buttons};

impl SessionWriter {
    pub(super) fn write_command(&mut self, command: ClientCommand) -> io::Result<()> {
        match command {
            ClientCommand::Pointer { x, y, buttons } => {
                self.pointer_buttons = buttons;
                let sent = if self.apple_key.is_some() {
                    apple_pointer_buttons(buttons)
                } else {
                    buttons
                };
                write_pointer_event(&mut self.output, sent, x, y)
            }
            ClientCommand::Key { keysym, down } => self.write_key(keysym, down),
            ClientCommand::Scroll { x, y, right, down } => {
                self.write_scroll(AppleScroll { x, y, right, down })
            }
            ClientCommand::ClipboardText(text) => self.write_clipboard(text),
            ClientCommand::RefreshScreen => self.refresh_screen(),
            ClientCommand::SetEncodings(encodings) => self.write_encodings(&encodings),
            ClientCommand::ResizeDisplay { width, height } => {
                if let Some(display) = &mut self.display {
                    display.resize(width, height);
                }
                Ok(())
            }
            ClientCommand::PlaySound(play) => {
                if let Some(display) = &self.display {
                    display.peer.play_sound(play);
                }
                Ok(())
            }
            ClientCommand::SetInputSource(source) if self.apple_key.is_some() => {
                write_apple_set_input_source(&mut self.output, &source)
            }
            ClientCommand::SetInputSource(_) => Ok(()),
        }
    }
}
