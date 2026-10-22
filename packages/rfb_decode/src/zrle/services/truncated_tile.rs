use crate::DecodeError;

pub(crate) fn truncated_tile() -> DecodeError {
    DecodeError::Corrupt("a ZRLE tile runs past the end of the inflated data".to_owned())
}
