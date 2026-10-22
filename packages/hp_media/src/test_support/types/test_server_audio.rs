use super::TestServer;
use crate::test_support::{TEST_AUDIO_PAYLOAD_TYPE, TEST_AUDIO_SSRC};

impl TestServer {
    pub fn audio(&mut self) -> Vec<u8> {
        let mut plain = vec![0x80, TEST_AUDIO_PAYLOAD_TYPE];
        plain.extend_from_slice(&self.audio_seq.to_be_bytes());
        plain.extend_from_slice(&u32::from(self.audio_seq).to_be_bytes());
        plain.extend_from_slice(&TEST_AUDIO_SSRC.to_be_bytes());
        plain.extend_from_slice(&[0; 4]);
        self.audio_seq = self.audio_seq.wrapping_add(1);
        self.to_client[0]
            .protect_rtp(&plain)
            .expect("a test sound packet is protected")
    }
}
