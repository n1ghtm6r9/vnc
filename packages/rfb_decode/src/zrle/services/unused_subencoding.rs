use crate::DecodeError;

pub(crate) fn unused_subencoding(subencoding: u8) -> DecodeError {
    DecodeError::Corrupt(format!(
        "the ZRLE subencoding {subencoding} is not used by the protocol"
    ))
}
