use platform::GrabbedKey;
use rfb_session::ClientCommand;

use super::InputRouter;
use crate::physical_key_for_virtual_key;

impl InputRouter {
    pub fn grabbed_key(&mut self, key: GrabbedKey) -> Option<ClientCommand> {
        let physical = physical_key_for_virtual_key(key.virtual_key)?;
        self.key(physical, None, key.down)
    }
}
