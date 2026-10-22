use std::time::Instant;

use ard_media::{MediaAnswer, MediaSecrets, SrtpContext, VideoAssembler, split_key_block};

use super::MediaStream;

impl MediaStream {
    pub(crate) fn new(secrets: MediaSecrets, answer: MediaAnswer, now: Instant) -> MediaStream {
        let audio = split_key_block(&secrets.audio_keys);
        let video = split_key_block(&secrets.video_keys);
        MediaStream {
            incoming: [
                SrtpContext::new(&audio.from_server),
                SrtpContext::new(&video.from_server),
            ],
            outgoing: [
                SrtpContext::new(&audio.to_server),
                SrtpContext::new(&video.to_server),
            ],
            assembler: VideoAssembler::new(answer.video.ssrc, answer.video.stripes),
            highest_seq: [None; 2],
            sender_reports: [None; 2],
            last_media_at: now,
            last_video_packet_at: None,
            last_picture_at: now,
            shown: false,
            secrets,
            answer,
        }
    }
}
