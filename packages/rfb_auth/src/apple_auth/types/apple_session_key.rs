use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct AppleSessionKey(pub(crate) [u8; 16]);

impl fmt::Debug for AppleSessionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AppleSessionKey(***)")
    }
}
