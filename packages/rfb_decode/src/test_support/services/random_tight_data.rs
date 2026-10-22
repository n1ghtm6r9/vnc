use super::random_bytes;
use crate::test_support::ZlibFixture;

pub fn random_tight_data(state: &mut u64, length: usize) -> Vec<u8> {
    let content = random_bytes(state, length);
    if length < 12 {
        return content;
    }
    ZlibFixture::new().tight_payload(&content)
}
