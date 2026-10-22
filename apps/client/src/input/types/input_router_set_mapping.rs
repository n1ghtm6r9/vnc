use remote_input::KeyMapping;
use rfb_session::ClientCommand;

use super::InputRouter;

impl InputRouter {
    pub fn set_mapping(&mut self, mapping: KeyMapping) -> Vec<ClientCommand> {
        let released = self.release_all();
        self.mapping = mapping;
        released
    }
}
