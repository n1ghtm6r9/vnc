use super::{read_array, read_slice, read_varint};
use crate::{MediaError, ProtoField, ProtoValue};

pub fn decode_proto(bytes: &[u8]) -> Result<Vec<ProtoField>, MediaError> {
    let mut fields = Vec::new();
    let mut position = 0;
    while position < bytes.len() {
        let tag = read_varint(bytes, &mut position)?;
        let number = u32::try_from(tag >> 3)
            .ok()
            .filter(|number| *number != 0)
            .ok_or_else(|| MediaError::Malformed("protocol buffer field number".into()))?;
        let value = match tag & 7 {
            0 => ProtoValue::Varint(read_varint(bytes, &mut position)?),
            1 => ProtoValue::Fixed64(read_array(bytes, &mut position)?),
            2 => {
                let length = read_varint(bytes, &mut position)?;
                ProtoValue::Bytes(read_slice(bytes, &mut position, length)?.to_vec())
            }
            5 => ProtoValue::Fixed32(read_array(bytes, &mut position)?),
            wire => {
                return Err(MediaError::Malformed(format!(
                    "protocol buffer wire type {wire}"
                )));
            }
        };
        fields.push(ProtoField { number, value });
    }
    Ok(fields)
}
