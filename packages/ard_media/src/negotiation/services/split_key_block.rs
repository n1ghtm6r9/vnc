use crate::{KEY_BLOCK_LENGTH, KeyBlock, MasterKey};

pub fn split_key_block(block: &[u8; KEY_BLOCK_LENGTH]) -> KeyBlock {
    KeyBlock {
        to_server: MasterKey {
            key: block[0..32].try_into().expect("32 bytes"),
            salt: block[32..46].try_into().expect("14 bytes"),
        },
        from_server: MasterKey {
            key: block[46..78].try_into().expect("32 bytes"),
            salt: block[78..92].try_into().expect("14 bytes"),
        },
    }
}
