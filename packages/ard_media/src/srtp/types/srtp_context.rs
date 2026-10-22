use std::collections::HashMap;

use super::SrtpCipher;

pub struct SrtpContext {
    pub(crate) rtp: SrtpCipher,
    pub(crate) rtcp: SrtpCipher,
    pub(crate) received: HashMap<u32, u64>,
    pub(crate) sent: HashMap<u32, u64>,
    pub(crate) rtcp_index: u32,
}
