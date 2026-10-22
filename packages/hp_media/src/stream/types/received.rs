use ard_media::RtpPacket;

pub(crate) enum Received<'a> {
    Rejected,
    Control,
    Audio(RtpPacket<'a>),
    Video(RtpPacket<'a>),
}
