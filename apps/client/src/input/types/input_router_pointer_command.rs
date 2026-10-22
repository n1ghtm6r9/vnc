use rfb_session::ClientCommand;

use super::InputRouter;

impl InputRouter {
    pub(super) fn pointer_command(&self, buttons: u8) -> ClientCommand {
        let (x, y) = self.position.unwrap_or_default();
        ClientCommand::Pointer { x, y, buttons }
    }
}
