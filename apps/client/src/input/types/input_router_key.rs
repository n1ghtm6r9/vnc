use remote_input::keysym_for_typed_key;
use rfb_session::ClientCommand;
use winit::keyboard::PhysicalKey;

use super::InputRouter;

impl InputRouter {
    pub fn key(&mut self, key: PhysicalKey, character: Option<char>, down: bool) -> Option<ClientCommand> {
        if !down {
            return self.pressed.release(key).map(|keysym| ClientCommand::Key { keysym, down: false });
        }
        let shortcut_held = self.pressed.holds_shortcut_modifier();
        let keysym = keysym_for_typed_key(key, character, self.mapping, shortcut_held)?;
        let keysym = self.pressed.press(key, keysym);
        Some(ClientCommand::Key { keysym, down: true })
    }
}
