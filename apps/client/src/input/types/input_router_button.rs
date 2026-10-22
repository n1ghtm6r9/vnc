use remote_input::button_bit;
use rfb_session::ClientCommand;
use winit::event::MouseButton;

use super::InputRouter;

impl InputRouter {
    pub fn button(&mut self, button: MouseButton, down: bool) -> Option<ClientCommand> {
        let bit = button_bit(button)?;
        let buttons = if down { self.buttons | bit } else { self.buttons & !bit };
        if buttons == self.buttons {
            return None;
        }
        self.buttons = buttons;
        Some(self.pointer_command(buttons))
    }
}
