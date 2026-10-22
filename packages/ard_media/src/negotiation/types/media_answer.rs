use super::VideoFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaAnswer {
    pub version: u16,
    pub audio_ssrc: u32,
    pub video: VideoFormat,
}
