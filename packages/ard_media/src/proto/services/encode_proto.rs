use super::write_varint;
use crate::{ProtoField, ProtoValue};

pub fn encode_proto(fields: &[ProtoField]) -> Vec<u8> {
    let mut out = Vec::new();
    for field in fields {
        write_varint(
            &mut out,
            u64::from(field.number) << 3 | field.value.wire_type(),
        );
        match &field.value {
            ProtoValue::Varint(value) => write_varint(&mut out, *value),
            ProtoValue::Fixed64(bytes) => out.extend_from_slice(bytes),
            ProtoValue::Bytes(bytes) => {
                write_varint(&mut out, bytes.len() as u64);
                out.extend_from_slice(bytes);
            }
            ProtoValue::Fixed32(bytes) => out.extend_from_slice(bytes),
        }
    }
    out
}
