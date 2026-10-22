use crate::{ProtoField, ProtoValue};

pub fn proto_bytes(fields: &[ProtoField], number: u32) -> Option<&[u8]> {
    fields
        .iter()
        .find(|field| field.number == number)
        .and_then(|field| match &field.value {
            ProtoValue::Bytes(bytes) => Some(bytes.as_slice()),
            _ => None,
        })
}
