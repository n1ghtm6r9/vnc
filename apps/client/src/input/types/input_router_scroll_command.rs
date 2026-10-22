use remote_input::scroll_pixels;
use rfb_session::ClientCommand;
use winit::event::MouseScrollDelta;

use super::InputRouter;

impl InputRouter {
    pub(super) fn scroll_command(&self, delta: MouseScrollDelta) -> Option<ClientCommand> {
        let (right, down) = scroll_pixels(delta);
        if right == 0 && down == 0 {
            return None;
        }
        let (x, y) = self.position.unwrap_or_default();
        Some(ClientCommand::Scroll { x, y, right, down })
    }
}
