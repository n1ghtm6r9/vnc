use super::ProtoField;

impl ProtoField {
    pub fn text(number: u32, value: &str) -> Self {
        Self::bytes(number, value.as_bytes().to_vec())
    }
}
