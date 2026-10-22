use crate::{AppleSessionKey, Credentials};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppleLogin {
    pub credentials: Credentials,
    pub key: AppleSessionKey,
}
