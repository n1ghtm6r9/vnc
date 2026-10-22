#[cfg(windows)]
pub(crate) fn password_from_blob(blob: &[u8]) -> String {
    let units: Vec<u16> = blob
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| u16::from_le_bytes(*pair))
        .collect();
    String::from_utf16_lossy(&units)
}
