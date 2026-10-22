use ard_media::{PAYLOAD_TYPE_HEVC, SrtpContext, split_key_block};

use super::TestServer;
use crate::test_support::{FIRST_SEQ, fixed_secrets};

impl TestServer {
    pub fn new() -> TestServer {
        let secrets = fixed_secrets().expect("the fixed secrets are made");
        let audio = split_key_block(&secrets.audio_keys);
        let video = split_key_block(&secrets.video_keys);
        TestServer {
            to_client: [
                SrtpContext::new(&audio.from_server),
                SrtpContext::new(&video.from_server),
            ],
            from_client: [
                SrtpContext::new(&audio.to_server),
                SrtpContext::new(&video.to_server),
            ],
            seq: FIRST_SEQ,
            audio_seq: FIRST_SEQ,
            payload_type: PAYLOAD_TYPE_HEVC,
        }
    }
}
