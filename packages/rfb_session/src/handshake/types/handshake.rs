use rfb::ServerInit;
use rfb_auth::AppleSessionKey;

pub(crate) struct Handshake {
    pub(crate) init: ServerInit,
    pub(crate) apple_key: Option<AppleSessionKey>,
    pub(crate) capabilities: Option<[u8; 16]>,
}
