use super::ProtoValue;

impl ProtoValue {
    pub fn wire_type(&self) -> u64 {
        match self {
            ProtoValue::Varint(_) => 0,
            ProtoValue::Fixed64(_) => 1,
            ProtoValue::Bytes(_) => 2,
            ProtoValue::Fixed32(_) => 5,
        }
    }
}
