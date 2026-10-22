use rfb_session::ClientCommand;
use winit::event::MouseScrollDelta;

use super::InputRouter;

impl InputRouter {
    pub fn wheel(&mut self, delta: MouseScrollDelta) -> Vec<ClientCommand> {
        if self.apple_scroll {
            return self.scroll_command(delta).into_iter().collect();
        }
        self.wheel
            .push(delta)
            .into_iter()
            .flat_map(|step| {
                [
                    self.pointer_command(self.buttons | step.button_bit()),
                    self.pointer_command(self.buttons),
                ]
            })
            .collect()
    }
}
