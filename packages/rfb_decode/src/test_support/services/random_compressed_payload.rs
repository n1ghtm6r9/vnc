use super::{length_prefixed, random_below, random_bytes};
use crate::test_support::ZlibFixture;

pub fn random_compressed_payload(state: &mut u64) -> Vec<u8> {
    let length = random_below(state, 2000);
    let content = random_bytes(state, length);
    length_prefixed(&ZlibFixture::new().compress(&content))
}
