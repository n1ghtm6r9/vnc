use super::with_length;

pub fn message1_payload() -> Vec<u8> {
    let strings = ["Mac16,1", "unknown", "unknown"];
    let mut body = vec![0, 2, 0, 0, 0, 1, 0, 0, 0, 0];
    for text in strings {
        body.extend_from_slice(&(text.len() as u16 + 1).to_be_bytes());
    }
    for text in strings {
        body.extend_from_slice(text.as_bytes());
        body.push(0);
    }
    body.extend_from_slice(&[0; 4]);
    with_length(body)
}
