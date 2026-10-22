use crate::DecodeError;

pub(crate) fn bitstream_ended() -> DecodeError {
    DecodeError::Corrupt("the MVS bitstream ends early".to_owned())
}
