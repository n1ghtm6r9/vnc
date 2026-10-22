use super::rtp_extension;
use crate::test_support::{TEST_BASE_SSRC, TestPacket};

pub fn single_unit_packet(stripe: u32, don: u16, kind: u8, body: &[u8]) -> TestPacket {
    let mut payload = vec![kind << 1, 1];
    payload.extend_from_slice(&don.to_be_bytes());
    payload.extend_from_slice(body);
    TestPacket {
        ssrc: TEST_BASE_SSRC + stripe,
        seq: don.wrapping_add(0x4000),
        extension: rtp_extension(1, 7),
        payload,
    }
}
