use super::{ProtoField, ProtoValue};

impl ProtoField {
    pub fn bytes(number: u32, value: Vec<u8>) -> Self {
        Self {
            number,
            value: ProtoValue::Bytes(value),
        }
    }
}
