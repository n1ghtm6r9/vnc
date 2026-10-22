use crate::{ProtoField, ProtoValue};

pub fn proto_varint(fields: &[ProtoField], number: u32) -> Option<u64> {
    fields
        .iter()
        .find(|field| field.number == number)
        .and_then(|field| match field.value {
            ProtoValue::Varint(value) => Some(value),
            _ => None,
        })
}
