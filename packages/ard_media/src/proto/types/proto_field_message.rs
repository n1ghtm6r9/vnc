use super::ProtoField;
use crate::encode_proto;

impl ProtoField {
    pub fn message(number: u32, fields: &[ProtoField]) -> Self {
        Self::bytes(number, encode_proto(fields))
    }
}
