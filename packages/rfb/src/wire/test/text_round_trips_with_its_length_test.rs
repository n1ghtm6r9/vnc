use crate::{read_text, write_text};

#[test]
fn test_text_round_trips_with_its_length() {
    let mut bytes = Vec::new();
    write_text(&mut bytes, "MacBook Pro".as_bytes()).unwrap();
    assert_eq!(&bytes[..4], &[0, 0, 0, 11], "the length goes first as a big-endian u32");
    assert_eq!(read_text(&mut &bytes[..]).unwrap(), "MacBook Pro", "the text reads back unchanged");
}
