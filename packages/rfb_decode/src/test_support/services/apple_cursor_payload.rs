pub fn apple_cursor_payload(id: u32, data: &[u8]) -> Vec<u8> {
    let length = u32::try_from(data.len()).expect("a test cursor fits a u32 length");
    let mut payload = id.to_be_bytes().to_vec();
    payload.extend_from_slice(&length.to_be_bytes());
    payload.extend_from_slice(data);
    payload
}
