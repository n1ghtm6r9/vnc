#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasterKey {
    pub key: [u8; 32],
    pub salt: [u8; 14],
}
