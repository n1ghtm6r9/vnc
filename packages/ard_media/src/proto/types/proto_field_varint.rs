use super::{ProtoField, ProtoValue};

impl ProtoField {
    pub fn varint(number: u32, value: u64) -> Self {
        Self {
            number,
            value: ProtoValue::Varint(value),
        }
    }
}
