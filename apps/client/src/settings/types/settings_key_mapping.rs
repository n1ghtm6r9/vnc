use remote_input::{CommandKeysym, KeyMapping};

use super::Settings;

impl Settings {
    pub fn key_mapping(&self) -> KeyMapping {
        KeyMapping {
            swap_control_and_command: self.swap_control_and_command,
            command_keysym: CommandKeysym::Super,
        }
    }
}
