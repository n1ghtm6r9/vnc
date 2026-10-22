use super::TestServer;
use crate::{
    MediaChannel,
    test_support::{TEST_AUDIO_SSRC, TEST_VIDEO_SSRC},
};

impl TestServer {
    pub fn sender_report(&mut self, channel: MediaChannel, middle: u32) -> Vec<u8> {
        let ssrc = match channel {
            MediaChannel::Audio => TEST_AUDIO_SSRC,
            MediaChannel::Video => TEST_VIDEO_SSRC,
        };
        let mut plain = vec![0x80, 200, 0, 6];
        plain.extend_from_slice(&ssrc.to_be_bytes());
        plain.extend_from_slice(&[0, 0]);
        plain.extend_from_slice(&middle.to_be_bytes());
        plain.extend_from_slice(&[0; 14]);
        self.to_client[channel.index()]
            .protect_rtcp(&plain)
            .expect("a sender report is protected")
    }
}
