pub(crate) fn latin1_bytes(text: &str) -> Vec<u8> {
    text.chars()
        .map(|character| u8::try_from(character).unwrap_or(b'?'))
        .collect()
}
