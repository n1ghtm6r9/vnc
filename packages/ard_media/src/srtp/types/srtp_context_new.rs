use std::collections::HashMap;

use super::{MasterKey, SrtpCipher, SrtpContext};
use crate::derive_session_keys;

impl SrtpContext {
    pub fn new(master: &MasterKey) -> Self {
        let keys = derive_session_keys(master);
        Self {
            rtp: SrtpCipher::new(&keys.rtp),
            rtcp: SrtpCipher::new(&keys.rtcp),
            received: HashMap::new(),
            sent: HashMap::new(),
            rtcp_index: 0,
        }
    }
}
