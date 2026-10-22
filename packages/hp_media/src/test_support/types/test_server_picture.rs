use super::TestServer;
use crate::test_support::TEST_VIDEO_SSRC;

impl TestServer {
    pub fn picture(&mut self, don: u16, stripe: u32, units: &[&[u8]]) -> Vec<Vec<u8>> {
        let count = units.len() as u16;
        units
            .iter()
            .map(|unit| {
                let mut plain = vec![0x90, self.payload_type];
                plain.extend_from_slice(&self.seq.to_be_bytes());
                plain.extend_from_slice(&u32::from(don).to_be_bytes());
                plain.extend_from_slice(&(TEST_VIDEO_SSRC + stripe).to_be_bytes());
                plain.extend_from_slice(&[0x93, 0x11, 0x00, 0x01]);
                plain.extend_from_slice(&count.to_be_bytes());
                plain.extend_from_slice(&don.to_be_bytes());
                plain.extend_from_slice(&unit[..2]);
                plain.extend_from_slice(&don.to_be_bytes());
                plain.extend_from_slice(&unit[2..]);
                self.seq = self.seq.wrapping_add(1);
                self.to_client[1]
                    .protect_rtp(&plain)
                    .expect("a test packet is protected")
            })
            .collect()
    }
}
