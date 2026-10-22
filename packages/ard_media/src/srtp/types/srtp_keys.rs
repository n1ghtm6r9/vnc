#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrtpKeys {
    pub cipher_key: [u8; 32],
    pub auth_key: [u8; 20],
    pub salt: [u8; 14],
}
