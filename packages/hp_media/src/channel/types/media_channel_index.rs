use super::MediaChannel;

impl MediaChannel {
    pub(crate) fn index(self) -> usize {
        match self {
            MediaChannel::Audio => 0,
            MediaChannel::Video => 1,
        }
    }
}
