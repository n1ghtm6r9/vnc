use log::debug;
use rfb::{ENCODING_APPLE_HIGH_PERFORMANCE, ENCODING_APPLE_MEDIA_STREAM};

use super::SessionReader;

impl SessionReader {
    pub(super) fn pass_media(&self, encoding: i32, payload: Vec<u8>) {
        let Some(media) = &self.media else {
            debug!("[SESSION] the Mac sent media rect {encoding} to a session without High Performance");
            return;
        };
        match encoding {
            ENCODING_APPLE_MEDIA_STREAM => media.peer.message1(payload),
            ENCODING_APPLE_HIGH_PERFORMANCE => media.peer.answer(payload),
            other => debug!("[SESSION] the Mac sent media rect {other}, it is skipped"),
        }
    }
}
