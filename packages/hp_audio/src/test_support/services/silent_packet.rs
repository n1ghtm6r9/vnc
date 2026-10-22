use crate::SoundPacket;
use crate::test_support::{SILENT_FRAME, TEST_SSRC};

pub fn silent_packet(seq: u16) -> SoundPacket {
    SoundPacket {
        ssrc: TEST_SSRC,
        seq,
        payload: SILENT_FRAME.to_vec(),
    }
}
