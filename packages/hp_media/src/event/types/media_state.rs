use super::MediaFailure;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaState {
    Streaming {
        width: u32,
        height: u32,
        stripes: u32,
    },
    Failed(MediaFailure),
}
