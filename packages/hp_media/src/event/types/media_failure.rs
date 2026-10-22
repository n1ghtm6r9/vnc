use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaFailure {
    NoMessage1,
    BadMessage1(String),
    Offer(String),
    NoAnswer,
    BadAnswer(String),
    NoPicture,
    Decoder(String),
    Stalled,
    MediaStopped,
    UnsupportedVideo(u8),
    Socket(String),
}

impl fmt::Display for MediaFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaFailure::NoMessage1 => write!(f, "the Mac did not offer High Performance"),
            MediaFailure::BadMessage1(reason) => {
                write!(
                    f,
                    "the Mac's High Performance offer cannot be read: {reason}"
                )
            }
            MediaFailure::Offer(reason) => write!(f, "the media offer cannot be made: {reason}"),
            MediaFailure::NoAnswer => write!(f, "the Mac did not answer the media offer"),
            MediaFailure::BadAnswer(reason) => {
                write!(f, "the Mac's media answer cannot be read: {reason}")
            }
            MediaFailure::NoPicture => write!(f, "no picture came after the media answer"),
            MediaFailure::Decoder(reason) => write!(f, "the video decoder failed: {reason}"),
            MediaFailure::Stalled => write!(f, "the video stream stalled"),
            MediaFailure::MediaStopped => write!(f, "the media stream stopped"),
            MediaFailure::UnsupportedVideo(payload_type) => {
                write!(
                    f,
                    "the Mac sent video of payload type {payload_type}, not HEVC"
                )
            }
            MediaFailure::Socket(reason) => write!(f, "a media socket failed: {reason}"),
        }
    }
}
