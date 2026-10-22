use remote_input::{ViewRect, remote_point};
use rfb_session::ClientCommand;
use winit::dpi::{PhysicalPosition, PhysicalSize};

use super::InputRouter;

impl InputRouter {
    pub fn pointer_moved(
        &mut self,
        position: PhysicalPosition<f64>,
        view: ViewRect,
        remote: PhysicalSize<u16>,
    ) -> Option<ClientCommand> {
        let point = remote_point(position, view, remote);
        if self.position == Some(point) {
            return None;
        }
        self.position = Some(point);
        Some(self.pointer_command(self.buttons))
    }
}
