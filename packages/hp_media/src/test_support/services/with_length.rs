pub fn with_length(body: Vec<u8>) -> Vec<u8> {
    let mut payload = (body.len() as u16).to_be_bytes().to_vec();
    payload.extend(body);
    payload
}
