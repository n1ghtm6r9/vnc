use crate::APPLE_PASTEBOARD_TEXT_TYPE;

pub(crate) fn apple_pasteboard(text: &str) -> Vec<u8> {
    let field = |bytes: &mut Vec<u8>, data: &[u8]| {
        bytes.extend_from_slice(&(data.len() as u32).to_be_bytes());
        bytes.extend_from_slice(data);
    };
    let mut bytes = Vec::with_capacity(42 + text.len());
    bytes.extend_from_slice(&1u32.to_be_bytes());
    field(&mut bytes, APPLE_PASTEBOARD_TEXT_TYPE.as_bytes());
    bytes.extend_from_slice(&0u32.to_be_bytes());
    bytes.extend_from_slice(&0u32.to_be_bytes());
    field(&mut bytes, text.as_bytes());
    bytes
}
