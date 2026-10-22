use super::CommandKeysym;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct KeyMapping {
    pub swap_control_and_command: bool,
    pub command_keysym: CommandKeysym,
}
