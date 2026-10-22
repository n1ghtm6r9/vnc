use crate::{MAX_CUT_TEXT_LENGTH, ServerMessage};

#[test]
fn test_a_cut_text_longer_than_the_cap_is_skipped() {
    let length = MAX_CUT_TEXT_LENGTH + 1;
    let mut bytes = vec![3u8, 0, 0, 0];
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.resize(bytes.len() + length as usize, b'a');
    bytes.push(2);
    let mut stream = &bytes[..];
    let skipped = ServerMessage::read(&mut stream);
    assert!(matches!(skipped, Ok(ServerMessage::OversizedCutText { length: read }) if read == length), "a clipboard above 16 MiB is passed over without being kept, got {:?}", skipped.map(|_| ()));
    assert_eq!(ServerMessage::read(&mut stream).unwrap(), ServerMessage::Bell, "the message after the skipped clipboard is read as usual");
}
