use crate::{MediaError, ProtoField, proto_varint};

pub(crate) fn required_varint(
    fields: &[ProtoField],
    number: u32,
    what: &str,
) -> Result<u64, MediaError> {
    proto_varint(fields, number)
        .ok_or_else(|| MediaError::Malformed(format!("media answer without the {what}")))
}
