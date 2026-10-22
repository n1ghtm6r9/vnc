use crate::ServerInit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppleServerInit {
    pub init: ServerInit,
    pub flags: u32,
    pub capabilities: [u8; 16],
}
