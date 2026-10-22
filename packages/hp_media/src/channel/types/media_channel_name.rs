use super::MediaChannel;

impl MediaChannel {
    pub(crate) fn name(self) -> &'static str {
        match self {
            MediaChannel::Audio => "audio",
            MediaChannel::Video => "video",
        }
    }
}
