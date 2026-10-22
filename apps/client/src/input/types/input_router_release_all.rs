use rfb_session::ClientCommand;

use super::InputRouter;

impl InputRouter {
    pub fn release_all(&mut self) -> Vec<ClientCommand> {
        let mut commands: Vec<ClientCommand> = self
            .pressed
            .drain()
            .into_iter()
            .map(|keysym| ClientCommand::Key { keysym, down: false })
            .collect();
        if self.buttons != 0 {
            self.buttons = 0;
            commands.push(self.pointer_command(0));
        }
        commands
    }
}
