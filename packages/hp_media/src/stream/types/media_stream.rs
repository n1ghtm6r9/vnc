use std::time::Instant;

use ard_media::{MediaAnswer, MediaSecrets, SrtpContext, VideoAssembler};

use super::SenderReport;

pub(crate) struct MediaStream {
    pub(crate) secrets: MediaSecrets,
    pub(crate) answer: MediaAnswer,
    pub(crate) incoming: [SrtpContext; 2],
    pub(crate) outgoing: [SrtpContext; 2],
    pub(crate) assembler: VideoAssembler,
    pub(crate) highest_seq: [Option<i64>; 2],
    pub(crate) sender_reports: [Option<SenderReport>; 2],
    pub(crate) last_media_at: Instant,
    pub(crate) last_video_packet_at: Option<Instant>,
    pub(crate) last_picture_at: Instant,
    pub(crate) shown: bool,
}
